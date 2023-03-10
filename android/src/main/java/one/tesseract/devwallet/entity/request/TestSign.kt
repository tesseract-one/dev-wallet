package one.tesseract.devwallet.entity.request

import android.os.Parcelable
import kotlinx.parcelize.Parcelize

@Parcelize
data class TestSign(var transaction: String, val signature: String, var result: String) : Parcelable