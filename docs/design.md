# Video Spore Protocol

## Rational

Spore Protocol 限制内容体积为500kb，而 Video Spore Protocol 旨在在不改变 Spore Protocol 的理念和核心设计下，突破内容的体积限制，实现更大容量内容的支持，为用户提供更丰富的体验和更广泛的内容选择。

Spore Protocol 对内容的体积限制是最大 500kb 的原因将内容存放于 **Spore Cell**，以及 CKB 的区块大小限制为 597kb。

<!-- 其实从协议设计的视角，我也喜欢单一 cell 的设计，简洁；只是我们这次想从用户的视角，尝试去改进 Spore 协议 -->

我们试图通过解耦 Spore 的标识和存储来解决上述问题。**Spore Cell** 仅作为 Spore 标识，**Spore Segment Cell** 负责存储 Spore 的内容数据。

## Glossary

- **Spore Cell** 作为 Spore 唯一标识
- **Spore Segment Cell** 负责存储 Spore 的内容数据
- **Spore Content** 表示 **Spore Cell** 的 `output_data`，记录 Spore 的元数据 `[content-type, hash(content)]`
- **Binding Lifecycle Lock** 是一个新合约，用于绑定多个 cells，保证对应的 cells 拥有相同的生命周期
- **Spore Video Mutant** 是一个新的 Spore Mutant，也是用于绑定多个 cells，保证对应的 cells 拥有相同的生命周期

## Design

### 1. Decouple identity cell and storage cell

**Spore Cell** 只记录完整内容的哈希值；

**Spore Segment Cell** 存储内容数据 **Spore Content**，如果内容的体积比较大，可以切分分别放置于多个 **Spore Segment Cells**。

链上通过新增 **Spore Segment Cell** 以及将 **Spore Content** 的单独存放，达到解耦 **Spore Cell** 和 **Spore Content** 的目的，使得 Spore 的资产内容可以突破 500KB 的限制。

### 2. Binding identity cell and storage

**Spore Cell** 基于 Spore Mutant 机制，即 **Spore Video Mutant**，
- 要求 transfer spore 时所有对应的 **Spore Segment Cell**s 都未被消费（所以 transfer 交易必须将 **Spore Segment Cell**s 包括进 `cellDeps`）；
- 要求 melt spore 时所有对应的 **Spore Segment Cell**s 都同时被消费掉（所以 melt 交易必须将  **Spore Segment Cell**s 包括进 `inputs`）；

**Spore Segment Cell** 的 `lock` script，即 **Binding Lifecycle Lock**，
- `lock.args = <Spore Cell 的 type_hash`>， `type_hash` 已然覆盖了 Spore ID
- `lock.code = <Binding Lifecycle Lock>`
- 保证了 **Spore Segment Cell** 被花费的唯一条件是跟 **Spore Cell** 一同被 melt

<img width="1100" alt="image" src="https://github.com/video-spore-protocol/video-spore-protocol/assets/1870648/6cdf6580-820d-47f4-9468-bb820d6945d6">

<!-- RGB++ 通过同构绑定将 CKB Cell 和比特币资产绑定；BRC20 通过 return 0 将 UTXO 和 BRC20 资产绑定；Binding Lifecycle Lock 通过 lock script 将 spore cell 和存储 cell 绑定 -->

### 3. dApp index

dApp 根据 **Spore Cell** 的 `mutant` 字段判断是否为 Video Spore Protocol，如果是，则 dApp 根据其 `type_hash` 索引出 **Spore Segment Cell**s，并按序拼接出完整的资产内容。


## Implementation

拆分了 **Spore Cell** 和 **Spore Segment Cell** 之后，为了保证原子性，我们需要保证二者的一一配对关系，以及确保二者的生命周期一致。
我们通过 **Video Spore Mutant** 和 **Binding Lifecycle Lock** 来实现。

### Video Spore Mutant

从 **Spore Cell** 的视角，
在 transfer 时，要求将 **Spore Segment Cell**s 放置入 `cellDeps`；
在 melt 时，    要求将 **Spore Segment Cell**s 放置入 `inputs`。

我们将通过 Spore mutant 特性来保证该承诺。

```rust
VideoMutant = Cell {
    output_data: <
        func main() {
            if operation == "mint"     { exit(0) }
            if operation == "transfer" { exit(!all_spore_segment_cells_exist_in(Source::CellDep)) }>;
            if operation == "melt"     { exit(!all_spore_segment_cells_exist_in(Source::Input)) }>;
        }

        func all_spore_segment_cells_exist_in(source) -> bool {
            segments = filter_cell_deps(
                source: source,
                filter: {
                    type: null,
                    lock: BindingLifecycleLock{ args: <Spore's type_hash> }
                })
            complete_content = segments
                .sort(  |segment| segment.output_data.segment_index)
                .reduce(|segment, acc| acc ++ segment.segment_data)
            hash_of_complete_content = blake2b(complete_content)
            
            return hash_of_complete_content == SporeCell.output_data.data_hash
        }
    >
}

SporeCell = Cell {
    output_data:
        content_type: application/spore+video;[]mutant=<VideoSporeMutant>
             content: <data_hash>
    type:
      hash_type: "data1"
      code_hash: SPORE_DATA_HASH # hash of Spore's type script data hash
           args: TYPE_ID
}
```

### Binding Lifecycle Lock

从 **Spore Segment Cell** 的视角，我们要保证，仅在 melt **Spore Cell** 时，**Spore Segment Cell** 才能被消费。**Spore Segment Cell** 的 `lock` script 是 _BindingLifecycle_，一个新的 lock script:

```rust
BindingLifecycleLock = Script {
    code: < IF the corresponding Spore is beening melt THEN RETURN true >,
    args: <Spore's type_hash>
}

SporeSegmentCell = Cell {
    output_data: < <segment_Index:: u8>, <segment_Data> :: bytes >,
    type: null,
    lock: BindingLifecycleLock
}
```

## Alternative

其实从协议层的视角，先 mint **Spore Segment Cell**s，再 mint **Spore Cell**，**Spore Segment Cell**s 记录 Spore ID，**Spore Cell** 记录 **Spore Segment Cell**s 的 `outpoint`。这样是更优雅的设计，但是这种设计下，dApp 构造交易的逻辑比较复杂，在创建 **Spore Segment Cell**s 交易时就提前确定 Spore ID，而提前确定 Spore ID 就要求提前确定 **Spore Cell** 交易的 first input。

## Conclusion

1. Keep the original concepts and designs fundamentally.
2. Support for larger volume of spore.
3. Reduce the size and transaction fees of transfer operations.
