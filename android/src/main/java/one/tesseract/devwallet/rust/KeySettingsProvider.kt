package one.tesseract.devwallet.rust

import one.tesseract.devwallet.entity.KeySettings
import one.tesseract.crabdroid.RustObject

class KeySettingsProvider(handle: Long) : RustObject(handle) {
    external fun load(): KeySettings
    external fun save(settings: KeySettings)
}