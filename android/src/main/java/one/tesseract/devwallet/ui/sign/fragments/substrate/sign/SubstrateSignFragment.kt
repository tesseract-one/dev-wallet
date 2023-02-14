package one.tesseract.devwallet.ui.sign.fragments.substrate.sign

import androidx.lifecycle.ViewModelProvider
import android.os.Bundle
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import one.tesseract.devwallet.databinding.FragmentSubstrateSignBinding
import one.tesseract.devwallet.entity.request.SubstrateSign

class SubstrateSignFragment(private val request: SubstrateSign) : Fragment() {
    private var _binding: FragmentSubstrateSignBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val viewModel = ViewModelProvider(this).get(SubstrateSignViewModel::class.java)
        viewModel.request.value = request

        _binding = FragmentSubstrateSignBinding.inflate(inflater, container, false)
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