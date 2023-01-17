package one.tesseract.devwallet.rust

import one.tesseract.devwallet.interop.RustObject

class Core(handle: Long): RustObject(handle) {
    external fun testSettingsProvider(): TestSettingsProvider
}