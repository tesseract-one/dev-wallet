package one.tesseract.devwallet.ui.test

import android.util.Log
import androidx.lifecycle.*
import one.tesseract.devwallet.entity.TestSettings

class TestSettingsViewModel : ViewModel() {
    private val _cache = MutableLiveData<TestSettings>().apply {
        value = TestSettings("_signed_by_tesseract", "error")
    }
    val cache: MutableLiveData<TestSettings> = _cache

    private val _signature = MutableLiveData<String>().apply {
        value = cache.value?.signature
    }
    val signature: MutableLiveData<String> = _signature

    private val _invalidator = MutableLiveData<String>().apply {
        value = cache.value?.invalidator
    }
    val invalidator: MutableLiveData<String> = _invalidator

    val dirty: LiveData<Boolean> = Transformations.switchMap(signature) { signature ->
        Transformations.switchMap(invalidator) { invalidator ->
            Transformations.map(cache) { cache ->
                (!signature.equals(cache.signature)) || (!invalidator.equals(cache.invalidator))
            }
        }
    }

    var saved = cache.value!!

    fun save() {
        val settings = TestSettings(signature.value!!, invalidator.value!!)

        //TODO: write to file (in rust)
        saved = settings
        Log.d("SAVE", "Sig " + settings.signature)
        Log.d("SAVE", "Inv " + settings.invalidator)

        cache.value = settings
    }

    fun load() {
        //TODO: load from file (in rust)
        val loaded = saved

        cache.value = loaded
        signature.value = loaded.signature
        invalidator.value = loaded.invalidator
    }
}