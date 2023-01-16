package one.tesseract.devwallet.rust

import one.tesseract.devwallet.entity.TestSettings
import one.tesseract.devwallet.interop.RustObject

class TestSettingsProvider(handle: Long) : RustObject(handle) {
    external fun load(): TestSettings
    external fun save(settings: TestSettings)
}
