# OKX DEX V2 Parser

Instruction 對應這些 event，並且有 fallback。Primary 是我在鏈上看到的，fallback 是 event type 存在但是沒有看到有交易，但可能因為太少沒看到，所以我先放著以防萬一。

| Instruction | Primary Event | Fallback 1 | Fallback 2 | NOTE |
| --- | --- | --- | --- | --- |
| Swap | SwapCpiEvent2 | SwapWithFeesCpiEvent | SwapWithFeesCpiEvent2 | --- |
| ProxySwap | SwapWithFeesCpiEvent | SwapToBWithFeesCpiEventV2 | SwapWithFeesCpiEventEnhanced2 | no real tx yet |
| SwapTob | SwapWithFeesCpiEvent | SwapToBWithFeesCpiEventV2 | SwapWithFeesCpiEventEnhanced2 | |
| SwapTobEnhanced | SwapWithFeesCpiEventEnhanced | SwapToBWithFeesCpiEventV2 | SwapWithFeesCpiEventEnhanced2 | |
| SwapTobV2 | SwapWithFeesCpiEvent | SwapTobV2CpiEvent2 | SwapToBWithFeesCpiEventV2 | no real tx yet |
| SwapTobWithReceiver | SwapWithFeesCpiEvent2 | SwapToBWithFeesCpiEventV2 | SwapWithFeesCpiEventEnhanced2 | |
| SwapToc | SwapWithFeesCpiEvent2 | SwapToCWithFeesCpiEventV2 | SwapWithFeesCpiEventEnhanced2 | |
| SwapTocV2 | SwapTocV2CpiEvent2 | SwapToCWithFeesCpiEventV2 | SwapWithFeesCpiEventEnhanced2 | |
