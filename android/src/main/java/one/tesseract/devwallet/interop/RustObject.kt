package one.tesseract.devwallet.interop

open class RustObject(private val handle: Long) {
    private external fun drop()

    protected fun finalize() {
        drop()
    }
}