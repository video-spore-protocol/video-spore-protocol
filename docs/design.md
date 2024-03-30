# Video Spore Protocol

## Rational

Spore Protocol 限制内容体积为500kb，而 Video Spore Protocol 旨在在不改变 Spore Protocol 的理念和核心设计下，突破内容的体积限制，实现更大容量内容的支持，为用户提供更丰富的体验和更广泛的内容选择。

## Motivation

解释 Spore Protocol 对于 Spore 500kb 限制的背后原因
1. Spore 将内容存放于 Spore Cell
2. CKB block size limit 为 597kb
## Glossary

- **Spore Cell**
- **Spore Content**: `SporeCell.content-type | SporeCell.content`
- **Spore Segment**
- **Spore Segment Hash**
- **Spore Segment Cell**

## Design

**Spore Content** 从 **Spore Cell** 中剥离出来，存放于单独的 **Spore Segment Cell**，如果内容的体积比较大，可以切分成多个 **Spore Segment Cells**；**Spore Cell** 只使用有序列表记录 **Spore Segments** 的哈希值。

链上通过新增 **Spore Segment Cell** 以及将 **Spore Content** 的单独存放，达到解耦 **Spore Cell** 和 **Spore Content** 的目的，使得 Spore 的资产内容可以突破 500KB 的限制。

链下 dApp 服务器在检索到 **Spore Cell** 时，根据 `mutant` 字段判断其是否指定 Video Spore Protocol。对于指定了 Video Spore Protocol 的 **Spore Cell**，服务器再根据 `SPORE_ID` 和 **Spore Segments Hash**es 获取到 **Spore Segment Cells**，最后拼接出完整的资产内容。

Video Spore Protocol 只是对 Spore Protocol 做了拓展，保持原有的理念和设计。
1. Redeemable Intrinsic Value
2. True On-Chain Ownership with Privacy
3. Multi Content Type 👉 even more content types
4. Zero-Fee Transfers

拆分了 **Spore Cell** 和 **Spore Segment Cell** 之后，为了保证原子性，我们需要保证二者的一一配对关系，以及确保二者的生命周期一致。做法是：
- 从 **Spore Segment Cell** 的角度，我们要保证，仅在 melt **Spore Cell** 时，**Spore Segment Cell** 才能被消费。**Spore Segment Cell** 的 `lock` script 设计为 _CellBindingScript_:
```rust
CellBindingLock = Script {
	code: <RETURN "yes" IF ("melt" operation) AND (Spore ID IN inputs) ELSE RETURN "no">
	args: <Spore ID>
}
```
- 从 **Spore Cell** 的角度，在 transfer 时，要求将 **Spore Segment Cell** 放置入 `cellDeps`；在 melt 时，要求将 **Spore Segment Cell** 放置入 `inputs`。这个是通过 mutant 特性来实现。
```rust
VideoMutant = Cell {
	output_data: <Lua:
					 IF operation == "mint", pass
					 IF operation == "transfer", make sure all the []data_hash exist in the cellDeps
					 IF operation == "melt", make sure all the []data_hash exist in the inputs
				 >
	...
}

SporeCell = Cell {
	output_data:
		content_type: application/video;mutant=<VideoMutant>
		content: []data_hash
	...
}
```

其实从协议层的视角，先 mint **Spore Segment Cell**s，再 mint **Spore Cell**，**Spore Segment Cell**s 记录 Spore ID，**Spore Cell** 记录 **Spore Segment Cell**s 的 `outpoint`。这样是更优雅的设计，但是这种设计下，链下构造交易的逻辑比较复杂，即在创建 **Spore Segment Cell**s 交易时就提前确定 Spore ID，而提前确定 Spore ID 就要求提前确定 **Spore Cell** 交易的 first input。

- [ ] 索引层：
1. 根据用户的 lock script 和 Spore type script 索引到 Spore Cell，再根据 ??? 索引到 Spore Segment Cell
2. 拼接多个 Spore Segments 成完整的 Spore Content

## Conclusion

1. Break out the size limit of Spore Protocol
2. Reduce the transaction fee when transfer because the transaction size has been reduced
