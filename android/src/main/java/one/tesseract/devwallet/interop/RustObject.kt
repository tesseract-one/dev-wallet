package one.tesseract.devwallet.interop

open class RustObject(protected val handle: Long) {
    private external fun drop()

    protected fun finalize() {
        drop()
    }
}