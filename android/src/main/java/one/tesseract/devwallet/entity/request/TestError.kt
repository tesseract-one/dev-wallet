package one.tesseract.devwallet.entity.request

import android.os.Parcelable
import kotlinx.parcelize.Parcelize

@Parcelize
data class TestError(var transaction: String, var error: String) : Parcelable