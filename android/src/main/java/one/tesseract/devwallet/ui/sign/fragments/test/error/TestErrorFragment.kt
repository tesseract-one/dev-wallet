package one.tesseract.devwallet.ui.sign.fragments.test.error

import androidx.lifecycle.ViewModelProvider
import android.os.Bundle
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import one.tesseract.devwallet.databinding.FragmentTestErrorBinding
import one.tesseract.devwallet.entity.request.TestError

class TestErrorFragment(private val request: TestError) : Fragment() {
    private var _binding: FragmentTestErrorBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val viewModel = ViewModelProvider(this).get(TestErrorViewModel::class.java)
        viewModel.request.value = request

        _binding = FragmentTestErrorBinding.inflate(inflater, container, false)
        binding.model = viewModel
        binding.lifecycleOwner = this

        //here

        return binding.root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}