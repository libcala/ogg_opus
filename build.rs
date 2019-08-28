use cc;

fn main() {
    // Compile Opus
    cc::Build::new()
        .file("lib/opus/analysis.c")
        .file("lib/opus/mapping_matrix.c")
        .file("lib/opus/mlp.c")
        .file("lib/opus/mlp_data.c")
        .file("lib/opus/opus.c")
        .file("lib/opus/opus_decoder.c")
        .file("lib/opus/opus_encoder.c")
        .file("lib/opus/opus_multistream.c")
        .file("lib/opus/opus_multistream_decoder.c")
        .file("lib/opus/opus_multistream_encoder.c")
        .file("lib/opus/opus_projection_decoder.c")
        .file("lib/opus/opus_projection_encoder.c")
        .file("lib/opus/repacketizer.c")
        .file("lib/opus/celt/bands.c")
        .file("lib/opus/celt/celt.c")
        .file("lib/opus/celt/celt_decoder.c")
        .file("lib/opus/celt/celt_encoder.c")
        .file("lib/opus/celt/celt_lpc.c")
        .file("lib/opus/celt/entenc.c")
        .file("lib/opus/celt/cwrs.c")
        .file("lib/opus/celt/entcode.c")
        .file("lib/opus/celt/entdec.c")
        .file("lib/opus/celt/laplace.c")
        .file("lib/opus/celt/mathops.c")
        .file("lib/opus/celt/mdct.c")
        .file("lib/opus/celt/modes.c")
        .file("lib/opus/celt/pitch.c")
        .file("lib/opus/celt/quant_bands.c")
        .file("lib/opus/celt/vq.c")
        .file("lib/opus/celt/rate.c")
        .file("lib/opus/celt/kiss_fft.c")
        .file("lib/opus/silk/CNG.c")
        .file("lib/opus/silk/NSQ.c")
        .file("lib/opus/silk/PLC.c")
        .file("lib/opus/silk/VAD.c")
        .file("lib/opus/silk/sort.c")
        .file("lib/opus/silk/debug.c")
        .file("lib/opus/silk/A2NLSF.c")
        .file("lib/opus/silk/NLSF2A.c")
        .file("lib/opus/silk/dec_API.c")
        .file("lib/opus/silk/enc_API.c")
        .file("lib/opus/silk/lin2log.c")
        .file("lib/opus/silk/log2lin.c")
        .file("lib/opus/silk/LPC_fit.c")
        .file("lib/opus/silk/NLSF_VQ.c")
        .file("lib/opus/silk/sigm_Q15.c")
        .file("lib/opus/silk/resampler.c")
        .file("lib/opus/silk/biquad_alt.c")
        .file("lib/opus/silk/bwexpander.c")
        .file("lib/opus/silk/code_signs.c")
        .file("lib/opus/silk/gain_quant.c")
        .file("lib/opus/silk/tables_LTP.c")
        .file("lib/opus/silk/VQ_WMat_EC.c")
        .file("lib/opus/silk/control_SNR.c")
        .file("lib/opus/silk/decode_core.c")
        .file("lib/opus/silk/interpolate.c")
        .file("lib/opus/silk/NLSF_decode.c")
        .file("lib/opus/silk/NLSF_encode.c")
        .file("lib/opus/silk/NLSF_unpack.c")
        .file("lib/opus/silk/NSQ_del_dec.c")
        .file("lib/opus/silk/shell_coder.c")
        .file("lib/opus/silk/tables_gain.c")
        .file("lib/opus/silk/decode_frame.c")
        .file("lib/opus/silk/decode_pitch.c")
        .file("lib/opus/silk/init_decoder.c")
        .file("lib/opus/silk/init_encoder.c")
        .file("lib/opus/silk/tables_other.c")
        .file("lib/opus/silk/bwexpander_32.c")
        .file("lib/opus/silk/control_codec.c")
        .file("lib/opus/silk/decode_pulses.c")
        .file("lib/opus/silk/encode_pulses.c")
        .file("lib/opus/silk/process_NLSFs.c")
        .file("lib/opus/silk/resampler_rom.c")
        .file("lib/opus/silk/sum_sqr_shift.c")
        .file("lib/opus/silk/table_LSF_cos.c")
        .file("lib/opus/silk/decode_indices.c")
        .file("lib/opus/silk/decoder_set_fs.c")
        .file("lib/opus/silk/encode_indices.c")
        .file("lib/opus/silk/NLSF_stabilize.c")
        .file("lib/opus/silk/ana_filt_bank_1.c")
        .file("lib/opus/silk/quant_LTP_gains.c")
        .file("lib/opus/silk/resampler_down2.c")
        .file("lib/opus/silk/stereo_LR_to_MS.c")
        .file("lib/opus/silk/stereo_MS_to_LR.c")
        .file("lib/opus/silk/pitch_est_tables.c")
        .file("lib/opus/silk/tables_pitch_lag.c")
        .file("lib/opus/silk/decode_parameters.c")
        .file("lib/opus/silk/LPC_inv_pred_gain.c")
        .file("lib/opus/silk/resampler_down2_3.c")
        .file("lib/opus/silk/stereo_quant_pred.c")
        .file("lib/opus/silk/tables_NLSF_CB_WB.c")
        .file("lib/opus/silk/HP_variable_cutoff.c")
        .file("lib/opus/silk/inner_prod_aligned.c")
        .file("lib/opus/silk/LP_variable_cutoff.c")
        .file("lib/opus/silk/NLSF_del_dec_quant.c")
        .file("lib/opus/silk/stereo_decode_pred.c")
        .file("lib/opus/silk/stereo_encode_pred.c")
        .file("lib/opus/silk/check_control_input.c")
        .file("lib/opus/silk/LPC_analysis_filter.c")
        .file("lib/opus/silk/tables_NLSF_CB_NB_MB.c")
        .file("lib/opus/silk/resampler_private_AR2.c")
        .file("lib/opus/silk/stereo_find_predictor.c")
        .file("lib/opus/silk/NLSF_VQ_weights_laroia.c")
        .file("lib/opus/silk/control_audio_bandwidth.c")
        .file("lib/opus/silk/tables_pulses_per_block.c")
        .file("lib/opus/silk/resampler_private_up2_HQ.c")
        .file("lib/opus/silk/resampler_private_IIR_FIR.c")
        .file("lib/opus/silk/resampler_private_down_FIR.c")
        .file("lib/opus/celt/x86/x86cpu.c")
        .file("lib/opus/celt/x86/vq_sse2.c")
        .file("lib/opus/celt/x86/pitch_sse.c")
        .file("lib/opus/celt/x86/pitch_sse2.c")
        .file("lib/opus/celt/x86/pitch_sse4_1.c")
        .file("lib/opus/celt/x86/x86_celt_map.c")
        .file("lib/opus/celt/x86/celt_lpc_sse4_1.c")
        .file("lib/opus/silk/float/k2a_FLP.c")
        .file("lib/opus/silk/float/sort_FLP.c")
        .file("lib/opus/silk/float/schur_FLP.c")
        .file("lib/opus/silk/float/energy_FLP.c")
        .file("lib/opus/silk/float/find_LPC_FLP.c")
        .file("lib/opus/silk/float/find_LTP_FLP.c")
        .file("lib/opus/silk/float/wrappers_FLP.c")
        .file("lib/opus/silk/float/bwexpander_FLP.c")
        .file("lib/opus/silk/float/corrMatrix_FLP.c")
        .file("lib/opus/silk/float/encode_frame_FLP.c")
        .file("lib/opus/silk/float/scale_vector_FLP.c")
        .file("lib/opus/silk/float/burg_modified_FLP.c")
        .file("lib/opus/silk/float/inner_product_FLP.c")
        .file("lib/opus/silk/float/process_gains_FLP.c")
        .file("lib/opus/silk/float/LTP_scale_ctrl_FLP.c")
        .file("lib/opus/silk/float/autocorrelation_FLP.c")
        .file("lib/opus/silk/float/find_pitch_lags_FLP.c")
        .file("lib/opus/silk/float/find_pred_coefs_FLP.c")
        .file("lib/opus/silk/float/residual_energy_FLP.c")
        .file("lib/opus/silk/float/apply_sine_window_FLP.c")
        .file("lib/opus/silk/float/LPC_inv_pred_gain_FLP.c")
        .file("lib/opus/silk/float/scale_copy_vector_FLP.c")
        .file("lib/opus/silk/float/LPC_analysis_filter_FLP.c")
        .file("lib/opus/silk/float/LTP_analysis_filter_FLP.c")
        .file("lib/opus/silk/float/pitch_analysis_core_FLP.c")
        .file("lib/opus/silk/float/noise_shape_analysis_FLP.c")
        .file("lib/opus/silk/float/warped_autocorrelation_FLP.c")
        .file("lib/opus/silk/float/regularize_correlations_FLP.c")
        .file("lib/opus/silk/x86/NSQ_sse4_1.c")
        .file("lib/opus/silk/x86/VAD_sse4_1.c")
        .file("lib/opus/silk/x86/x86_silk_map.c")
        .file("lib/opus/silk/x86/VQ_WMat_EC_sse4_1.c")
        .file("lib/opus/silk/x86/NSQ_del_dec_sse4_1.c")
        .define("HAVE_CONFIG_H", None)
        .include("lib/opus")
        .include("lib/opus/celt")
        .include("lib/opus/silk")
        .include("lib/opus/silk/float")
        .flag("-O3")
        .flag("-msse4.1")
        .compile("opus");

    // Compile Opusenc
    cc::Build::new()
        .file("lib/opusenc/ogg_packer.c")
        .file("lib/opusenc/opusenc.c")
        .file("lib/opusenc/opus_header.c")
        .file("lib/opusenc/picture.c")
        .file("lib/opusenc/resample.c")
        .file("lib/opusenc/unicode_support.c")
        .define("PACKAGE_VERSION", Some("\"rust\""))
        .define("PACKAGE_NAME", Some("\"caved\""))
        .include("lib/opus")
        .compile("opusenc");
}