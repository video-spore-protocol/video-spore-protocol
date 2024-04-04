# Video Spore Protocol

Team: Kero @keroro520 , Lewis @kabudafax 

## Outline

1. Introduction to Spore Protocol and Video Spore Protocol
2. Architectural Design of Video Spore Protocol
3. Demo Time
4. Conclusion

<!-- 接下来我会按这四个部分介绍一下我们的成果。  -->
<!-- 第一部分我们会简单介绍 Spore 协议的特性和理念，然后引申出我们的拓展协议想要解决的问题  -->
<!-- 第二部分介绍拓展协议的协议设计 -->

## 1. Introduction to Spore Protocol and Video Spore Protocol


| Feature            | Spore                                                                                  | Video Spore                                                             |
| ------------------ | -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Basis of Value     | Intrinsic value + market value                                                         | Intrinsic value + market value                                          |
| Redeemability      | Yes                                                                                    | Yes                                                                     |
| Content            | On-chain                                                                               | On-chain                                                                |
| Immutability       | Yes, immutable                                                                         | Yes, immutable                                                          |
| “Mint” Fee         | Based on content size plus any extra value you want to infuse (redeemable)                | Based on data size plus any extra value you want to infuse (redeemable) |
| Transaction Cost   | ~~Zero-Fee Transfers, optional out-of-wallet payment in any token~~ Based on content size | Very low, independent of the content size                                                                |
| Content Size Limit | 500KB (essentially cell size limit)                                                    | Unlimited |
| Simplicity         | Simple, one cell for one digital object                                                | Spore Cell + Spore Segment Cell(s), for one digital object      |

‼️‼️ **Each transfer Spore transaction **contains the full spore content**, leading to large transaction sizes.**



## 2. Architectural Design of Video Spore Protocol

👉 [Protocol Design](https://github.com/video-spore-protocol/video-spore-protocol/blob/main/docs/design.md)

👉 [Progress Track](https://github.com/video-spore-protocol/video-spore-protocol/issues/1#issue-2216769341)

## 3. Demo

1. small video spore
2. large video spore


## 4. Conclusion

1. Keep the original concepts and designs fundamentally.
2. Support for larger volume of spore.
3. Reduce the size and transaction fees of transfer operations.


| Spore | Transfer Cost | Transfer TX Size | Tx |
| :--- | :--- | :--- | :--- |
| 16K spore | 0.00017052CKB (feeRate = 1000 shannons/KB) | 17,048 Bytes | [tx](https://pudge.explorer.nervos.org/transaction/0x2c504a7e7765860816ef5a85296046b60a5ef9560051bd8d00cfd3ed17faf5d8) |
| 16K video spore | 0.00001927CKB (feeRate = 1962 shannons/KB) | 978 Bytes | [tx](https://pudge.explorer.nervos.org/transaction/0x90fd61d84946631f119472c9e957244aea1cd77cc6edada111f9aeca1abe7628) |
| 286K spore | 0.00293274CKB (feeRate = 1000 shannons/KB) | 293,270 Bytes | [tx](https://pudge.explorer.nervos.org/transaction/0x12af0b38d225462be51b732ea4303da72d696c8cef053ef724cba0af5ad1531d) |
| 286K video spore | 0.00001927CKB (feeRate = 1962 shannons/KB) | 978 Bytes | [tx](https://pudge.explorer.nervos.org/transaction/0xd64534ddb217e7763d7bdd23a3d010cd03e094c6a52b900b35aae4e0c6bb12ed) |
| [900KB video spore](http://localhost:3000/spore/0x9f733552ea1047ae60129d777ed06e9cdc1ddff5ee110b25cb347201635b7ffb) | 0.00002001CKB (feeRate = 1894 shannons/KB) | 1,052 Bytes | [tx](https://pudge.explorer.nervos.org/transaction/0x9b7aeb6616c259a8cc444cfe253880e6e58c1a56f734c2b7dff6b10a995ddf1a) |

**Note**: Average fee rate on mainnet is 15,799 shannons/KB, visit https://explorer.nervos.org/fee-rate-tracker





