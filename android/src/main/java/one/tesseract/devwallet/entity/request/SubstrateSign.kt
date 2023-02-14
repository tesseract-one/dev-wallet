package one.tesseract.devwallet.entity.request

import android.os.Parcelable
import kotlinx.parcelize.Parcelize

@Parcelize
data class SubstrateSign(var algorithm: String, var path: String, var key: String, var data: String) : Parcelable