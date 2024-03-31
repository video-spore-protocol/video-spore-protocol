# Video Spore Protocol

## Rational

Spore Protocol 限制内容体积为500kb，而 Video Spore Protocol 旨在在不改变 Spore Protocol 的理念和核心设计下，突破内容的体积限制，实现更大容量内容的支持，为用户提供更丰富的体验和更广泛的内容选择。

## Glossary

- **Spore Cell**
- **Spore Content**: `SporeCell.content-type | SporeCell.content`
- **Spore Segment**
- **Spore Segment Cell**

## Design

Spore Protocol 对内容的体积限制是最大 500kb，该限制的原因 Spore protocol 将内容存放于 **Spore Cell**，以及 CKB 的区块大小限制为 597kb。

**Spore Content** 从 **Spore Cell** 中剥离出来，存放于单独的 **Spore Segment Cell**，如果内容的体积比较大，可以切分成多个 **Spore Segment Cells**；**Spore Cell** 只记录完整内容的哈希值，而 **Spore Segment Cell** 在 `lock.args` 里记录 **Spore Cell** 的 `type_hash`（note: `type_hash`已然覆盖了 Spore ID），用以构建绑定关系。

链上通过新增 **Spore Segment Cell** 以及将 **Spore Content** 的单独存放，达到解耦 **Spore Cell** 和 **Spore Content** 的目的，使得 Spore 的资产内容可以突破 500KB 的限制。

链下 dApp 服务器在检索到 **Spore Cell** 时，根据 `mutant` 字段判断其是否指定 Video Spore Protocol。对于指定了 Video Spore Protocol 的 **Spore Cell**，服务器根据其 `type_hash` 索引出 **Spore Segment Cell**s，并按序拼接出完整的资产内容。

Video Spore Protocol 只是对 Spore Protocol 做了拓展，保持原有的理念和设计。
1. Redeemable Intrinsic Value
2. True On-Chain Ownership with Privacy
3. Multi Content Type 👉 even more content types
4. Zero-Fee Transfers

拆分了 **Spore Cell** 和 **Spore Segment Cell** 之后，为了保证原子性，我们需要保证二者的一一配对关系，以及确保二者的生命周期一致。做法是：

- 从 **Spore Cell** 的角度，在 transfer 时，要求将 **Spore Segment Cell**s 放置入 `cellDeps`；在 melt 时，要求将 **Spore Segment Cell**s 放置入 `inputs`。我们将通过 mutant 特性来保证。

```rust
VideoMutant = Cell {
    output_data: < IF operation == "mint"     THEN RETURN true;
                   IF operation == "transfer" THEN RETURN <all the Spore Segment Cells exist in the cell_deps>;
                   IF operation == "melt"     THEN RETURN <all the Spore Segment Cells exist in the inputs> >

    // # snippet to check if all the Spore Segment Cells exist in the cell_deps:
    //
    // segments = filter_cell_deps({
    //     type: null,
    //     lock: BindingLifecycleLock{ args: <Spore's type_hash> }
    // )})
    // complete_content = segments
    //     .sort(  |segment| segment.output_data.segment_index)
    //     .reduce(|segment_data, acc| acc ++ segment.segment_data)
    // hash_of_complete_content = blake2b(complete_content)
    //
    // return hash_of_complete_content == SporeCell.output_data.data_hash

    ...
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

- 从 **Spore Segment Cell** 的角度，我们要保证，仅在 melt **Spore Cell** 时，**Spore Segment Cell** 才能被消费。**Spore Segment Cell** 的 `lock` script 是 _BindingLifecycle_，一个新的 lock script:

```rust
BindingLifecycleLock = Script {
    code: < IF operation == "melt" AND <the corresponding Spore is beening melt THEN RETURN true> >,
    args: <Spore's type_hash>
}

SporeSegmentCell = Cell {
    output_data: < <segment_Index:: u8>, <segment_Data> :: bytes >,
    type: null,
    lock: BindingLifecycleLock
}
```

其它做法：其实从协议层的视角，先 mint **Spore Segment Cell**s，再 mint **Spore Cell**，**Spore Segment Cell**s 记录 Spore ID，**Spore Cell** 记录 **Spore Segment Cell**s 的 `outpoint`。这样是更优雅的设计，但是这种设计下，链下构造交易的逻辑比较复杂，即在创建 **Spore Segment Cell**s 交易时就提前确定 Spore ID，而提前确定 Spore ID 就要求提前确定 **Spore Cell** 交易的 first input，链下的逻辑会很复杂。

## Conclusion

1. Surpass the size limitation imposed by the Spore Protocol.
2. Decrease the transaction fee during transfers as a result of a reduction in transaction size.
