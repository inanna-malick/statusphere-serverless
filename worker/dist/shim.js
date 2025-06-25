var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/worker/shim.mjs
import Ir from "./f0c7a8820fe90beb7ba63963bfbd5cca3b35071f-index.wasm";
import { WorkerEntrypoint as Tr } from "cloudflare:workers";
var B = Object.defineProperty;
var N = /* @__PURE__ */ __name((t2, e) => {
  for (var n in e) B(t2, n, { get: e[n], enumerable: true });
}, "N");
var p = {};
N(p, { IntoUnderlyingByteSource: /* @__PURE__ */ __name(() => I, "IntoUnderlyingByteSource"), IntoUnderlyingSink: /* @__PURE__ */ __name(() => T, "IntoUnderlyingSink"), IntoUnderlyingSource: /* @__PURE__ */ __name(() => F, "IntoUnderlyingSource"), MinifyConfig: /* @__PURE__ */ __name(() => E, "MinifyConfig"), MsgBroker: /* @__PURE__ */ __name(() => j, "MsgBroker"), PolishConfig: /* @__PURE__ */ __name(() => K, "PolishConfig"), R2Range: /* @__PURE__ */ __name(() => z, "R2Range"), RequestRedirect: /* @__PURE__ */ __name(() => Q, "RequestRedirect"), __wbg_String_8f0eb39a4a4c2f66: /* @__PURE__ */ __name(() => ot, "__wbg_String_8f0eb39a4a4c2f66"), __wbg_abort_410ec47a64ac6117: /* @__PURE__ */ __name(() => ct, "__wbg_abort_410ec47a64ac6117"), __wbg_abort_775ef1d17fc65868: /* @__PURE__ */ __name(() => it, "__wbg_abort_775ef1d17fc65868"), __wbg_acceptWebSocket_163b415f37889fa6: /* @__PURE__ */ __name(() => st, "__wbg_acceptWebSocket_163b415f37889fa6"), __wbg_accept_2c43be5ee3ef3652: /* @__PURE__ */ __name(() => ut, "__wbg_accept_2c43be5ee3ef3652"), __wbg_addEventListener_90e553fdce254421: /* @__PURE__ */ __name(() => ft, "__wbg_addEventListener_90e553fdce254421"), __wbg_all_8de32f92c8bbd12f: /* @__PURE__ */ __name(() => at, "__wbg_all_8de32f92c8bbd12f"), __wbg_append_8c7dd8d641a5f01b: /* @__PURE__ */ __name(() => bt, "__wbg_append_8c7dd8d641a5f01b"), __wbg_arrayBuffer_d1b44c4390db422f: /* @__PURE__ */ __name(() => gt, "__wbg_arrayBuffer_d1b44c4390db422f"), __wbg_bind_93c078d60dfda7d4: /* @__PURE__ */ __name(() => dt, "__wbg_bind_93c078d60dfda7d4"), __wbg_body_018617e858cb7195: /* @__PURE__ */ __name(() => wt, "__wbg_body_018617e858cb7195"), __wbg_body_0b8fd1fe671660df: /* @__PURE__ */ __name(() => pt, "__wbg_body_0b8fd1fe671660df"), __wbg_buffer_09165b52af8c5237: /* @__PURE__ */ __name(() => lt, "__wbg_buffer_09165b52af8c5237"), __wbg_buffer_609cc3eee51ed158: /* @__PURE__ */ __name(() => xt, "__wbg_buffer_609cc3eee51ed158"), __wbg_byobRequest_77d9adf63337edfb: /* @__PURE__ */ __name(() => mt, "__wbg_byobRequest_77d9adf63337edfb"), __wbg_byteLength_e674b853d9c77e1d: /* @__PURE__ */ __name(() => ht, "__wbg_byteLength_e674b853d9c77e1d"), __wbg_byteOffset_fd862df290ef848d: /* @__PURE__ */ __name(() => yt, "__wbg_byteOffset_fd862df290ef848d"), __wbg_call_672a4d21634d4a24: /* @__PURE__ */ __name(() => Rt, "__wbg_call_672a4d21634d4a24"), __wbg_call_7cccdd69e0791ae2: /* @__PURE__ */ __name(() => Ft, "__wbg_call_7cccdd69e0791ae2"), __wbg_call_833bed5770ea2041: /* @__PURE__ */ __name(() => St, "__wbg_call_833bed5770ea2041"), __wbg_call_b8adc8b1d0a0d8eb: /* @__PURE__ */ __name(() => kt, "__wbg_call_b8adc8b1d0a0d8eb"), __wbg_cancel_8a308660caa6cadf: /* @__PURE__ */ __name(() => It, "__wbg_cancel_8a308660caa6cadf"), __wbg_catch_a6e601879b2610e9: /* @__PURE__ */ __name(() => Tt, "__wbg_catch_a6e601879b2610e9"), __wbg_cause_9940c4e8dfcd5129: /* @__PURE__ */ __name(() => Et, "__wbg_cause_9940c4e8dfcd5129"), __wbg_cf_123509d53a2ea003: /* @__PURE__ */ __name(() => jt, "__wbg_cf_123509d53a2ea003"), __wbg_cf_abc51304c8a6868c: /* @__PURE__ */ __name(() => zt, "__wbg_cf_abc51304c8a6868c"), __wbg_clearTimeout_86721db0036bea98: /* @__PURE__ */ __name(() => Ot, "__wbg_clearTimeout_86721db0036bea98"), __wbg_close_2893b7d056a0627d: /* @__PURE__ */ __name(() => qt, "__wbg_close_2893b7d056a0627d"), __wbg_close_304cc1fef3466669: /* @__PURE__ */ __name(() => Mt, "__wbg_close_304cc1fef3466669"), __wbg_close_5ce03e29be453811: /* @__PURE__ */ __name(() => At, "__wbg_close_5ce03e29be453811"), __wbg_constructor_9fd96f589d65d4e5: /* @__PURE__ */ __name(() => Lt, "__wbg_constructor_9fd96f589d65d4e5"), __wbg_cron_232bea08224e3e32: /* @__PURE__ */ __name(() => Wt, "__wbg_cron_232bea08224e3e32"), __wbg_crypto_ed58b8e10a292839: /* @__PURE__ */ __name(() => vt, "__wbg_crypto_ed58b8e10a292839"), __wbg_data_432d9c3df2630942: /* @__PURE__ */ __name(() => Ct, "__wbg_data_432d9c3df2630942"), __wbg_debug_3cb59063b29f58c1: /* @__PURE__ */ __name(() => Ut, "__wbg_debug_3cb59063b29f58c1"), __wbg_done_769e5ede4b31c67b: /* @__PURE__ */ __name(() => Dt, "__wbg_done_769e5ede4b31c67b"), __wbg_enqueue_bb16ba72f537dc9e: /* @__PURE__ */ __name(() => Bt, "__wbg_enqueue_bb16ba72f537dc9e"), __wbg_entries_2a52db465d0421fb: /* @__PURE__ */ __name(() => Nt, "__wbg_entries_2a52db465d0421fb"), __wbg_error_0554b4a81edb112e: /* @__PURE__ */ __name(() => Pt, "__wbg_error_0554b4a81edb112e"), __wbg_error_524f506f44df1645: /* @__PURE__ */ __name(() => Vt, "__wbg_error_524f506f44df1645"), __wbg_error_7534b8e9a36f1ab4: /* @__PURE__ */ __name(() => $t, "__wbg_error_7534b8e9a36f1ab4"), __wbg_fetch_509096533071c657: /* @__PURE__ */ __name(() => Jt, "__wbg_fetch_509096533071c657"), __wbg_fetch_553598c1b6e65360: /* @__PURE__ */ __name(() => Gt, "__wbg_fetch_553598c1b6e65360"), __wbg_fetch_72dfa99e9fd87dd7: /* @__PURE__ */ __name(() => Ht, "__wbg_fetch_72dfa99e9fd87dd7"), __wbg_fetch_d36a73832f0a45e8: /* @__PURE__ */ __name(() => Xt, "__wbg_fetch_d36a73832f0a45e8"), __wbg_first_b0c1bd826f2fcb2d: /* @__PURE__ */ __name(() => Kt, "__wbg_first_b0c1bd826f2fcb2d"), __wbg_getRandomValues_bcb4912f16000dc4: /* @__PURE__ */ __name(() => Qt, "__wbg_getRandomValues_bcb4912f16000dc4"), __wbg_getReader_48e00749fe3f6089: /* @__PURE__ */ __name(() => Yt, "__wbg_getReader_48e00749fe3f6089"), __wbg_getTime_46267b1c24877e30: /* @__PURE__ */ __name(() => Zt, "__wbg_getTime_46267b1c24877e30"), __wbg_getWebSockets_f18105d7b88608ad: /* @__PURE__ */ __name(() => te, "__wbg_getWebSockets_f18105d7b88608ad"), __wbg_get_67b2ba62fc30de12: /* @__PURE__ */ __name(() => ee, "__wbg_get_67b2ba62fc30de12"), __wbg_get_85c3d71662a108c8: /* @__PURE__ */ __name(() => ne, "__wbg_get_85c3d71662a108c8"), __wbg_get_b0ab2e46022a2b56: /* @__PURE__ */ __name(() => re, "__wbg_get_b0ab2e46022a2b56"), __wbg_get_b9b93047fe3cf45b: /* @__PURE__ */ __name(() => _e, "__wbg_get_b9b93047fe3cf45b"), __wbg_getdone_d47073731acd3e74: /* @__PURE__ */ __name(() => oe, "__wbg_getdone_d47073731acd3e74"), __wbg_getvalue_009dcd63692bee1f: /* @__PURE__ */ __name(() => ce, "__wbg_getvalue_009dcd63692bee1f"), __wbg_getwithrefkey_1dc361bd10053bfe: /* @__PURE__ */ __name(() => ie, "__wbg_getwithrefkey_1dc361bd10053bfe"), __wbg_getwithrefkey_6550b2c093d2eb18: /* @__PURE__ */ __name(() => se, "__wbg_getwithrefkey_6550b2c093d2eb18"), __wbg_has_a5ea9117f258a0ec: /* @__PURE__ */ __name(() => ue, "__wbg_has_a5ea9117f258a0ec"), __wbg_headers_7852a8ea641c1379: /* @__PURE__ */ __name(() => fe, "__wbg_headers_7852a8ea641c1379"), __wbg_headers_9cb51cfd2ac780a4: /* @__PURE__ */ __name(() => ae, "__wbg_headers_9cb51cfd2ac780a4"), __wbg_httpProtocol_4cc3ab4fde2ecf82: /* @__PURE__ */ __name(() => be, "__wbg_httpProtocol_4cc3ab4fde2ecf82"), __wbg_idFromName_7b7b4a116f3a6ed2: /* @__PURE__ */ __name(() => ge, "__wbg_idFromName_7b7b4a116f3a6ed2"), __wbg_instanceof_ArrayBuffer_e14585432e3737fc: /* @__PURE__ */ __name(() => de, "__wbg_instanceof_ArrayBuffer_e14585432e3737fc"), __wbg_instanceof_Error_4d54113b22d20306: /* @__PURE__ */ __name(() => we, "__wbg_instanceof_Error_4d54113b22d20306"), __wbg_instanceof_Response_f2cc20d9f7dfd644: /* @__PURE__ */ __name(() => pe, "__wbg_instanceof_Response_f2cc20d9f7dfd644"), __wbg_instanceof_Uint8Array_17156bcf118086a9: /* @__PURE__ */ __name(() => le, "__wbg_instanceof_Uint8Array_17156bcf118086a9"), __wbg_isSafeInteger_343e2beeeece1bb0: /* @__PURE__ */ __name(() => xe, "__wbg_isSafeInteger_343e2beeeece1bb0"), __wbg_iterator_9a24c88df860dc65: /* @__PURE__ */ __name(() => me, "__wbg_iterator_9a24c88df860dc65"), __wbg_json_a00f187c0be01957: /* @__PURE__ */ __name(() => he, "__wbg_json_a00f187c0be01957"), __wbg_length_a446193dc22c12f8: /* @__PURE__ */ __name(() => ye, "__wbg_length_a446193dc22c12f8"), __wbg_length_e2d2a49132c1b256: /* @__PURE__ */ __name(() => Re, "__wbg_length_e2d2a49132c1b256"), __wbg_log_c222819a41e063d3: /* @__PURE__ */ __name(() => Fe, "__wbg_log_c222819a41e063d3"), __wbg_message_97a2af9b89d693a3: /* @__PURE__ */ __name(() => Se, "__wbg_message_97a2af9b89d693a3"), __wbg_method_3dcc854b644c5a56: /* @__PURE__ */ __name(() => ke, "__wbg_method_3dcc854b644c5a56"), __wbg_msCrypto_0a36e2ec3a343d26: /* @__PURE__ */ __name(() => Ie, "__wbg_msCrypto_0a36e2ec3a343d26"), __wbg_name_16617c8e9d4188ac: /* @__PURE__ */ __name(() => Te, "__wbg_name_16617c8e9d4188ac"), __wbg_new0_f788a2397c7ca929: /* @__PURE__ */ __name(() => Ee, "__wbg_new0_f788a2397c7ca929"), __wbg_new_018dcc2d6c8c2f6a: /* @__PURE__ */ __name(() => je, "__wbg_new_018dcc2d6c8c2f6a"), __wbg_new_23a2665fac83c611: /* @__PURE__ */ __name(() => ze, "__wbg_new_23a2665fac83c611"), __wbg_new_405e22f390576ce2: /* @__PURE__ */ __name(() => Oe, "__wbg_new_405e22f390576ce2"), __wbg_new_5e0be73521bc8c17: /* @__PURE__ */ __name(() => qe, "__wbg_new_5e0be73521bc8c17"), __wbg_new_78feb108b6472713: /* @__PURE__ */ __name(() => Me, "__wbg_new_78feb108b6472713"), __wbg_new_8a6f238a6ece86ea: /* @__PURE__ */ __name(() => Ae, "__wbg_new_8a6f238a6ece86ea"), __wbg_new_a12002a7f91c75be: /* @__PURE__ */ __name(() => Le, "__wbg_new_a12002a7f91c75be"), __wbg_new_c68d7209be747379: /* @__PURE__ */ __name(() => We, "__wbg_new_c68d7209be747379"), __wbg_new_e25e5aab09ff45db: /* @__PURE__ */ __name(() => ve, "__wbg_new_e25e5aab09ff45db"), __wbg_new_f20415acc2b509a9: /* @__PURE__ */ __name(() => Ce, "__wbg_new_f20415acc2b509a9"), __wbg_newnoargs_105ed471475aaf50: /* @__PURE__ */ __name(() => Ue, "__wbg_newnoargs_105ed471475aaf50"), __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a: /* @__PURE__ */ __name(() => De, "__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a"), __wbg_newwithintounderlyingsource_b47f6a6a596a7f24: /* @__PURE__ */ __name(() => Be, "__wbg_newwithintounderlyingsource_b47f6a6a596a7f24"), __wbg_newwithlength_a381634e90c276d4: /* @__PURE__ */ __name(() => Ne, "__wbg_newwithlength_a381634e90c276d4"), __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1: /* @__PURE__ */ __name(() => Pe, "__wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1"), __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e: /* @__PURE__ */ __name(() => Ve, "__wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e"), __wbg_newwithoptstrandinit_615a266ef226c260: /* @__PURE__ */ __name(() => $e, "__wbg_newwithoptstrandinit_615a266ef226c260"), __wbg_newwithstrandinit_06c535e0a867c635: /* @__PURE__ */ __name(() => Je, "__wbg_newwithstrandinit_06c535e0a867c635"), __wbg_next_25feadfc0913fea9: /* @__PURE__ */ __name(() => Ge, "__wbg_next_25feadfc0913fea9"), __wbg_next_6574e1a8a62d1055: /* @__PURE__ */ __name(() => He, "__wbg_next_6574e1a8a62d1055"), __wbg_node_02999533c4ea02e3: /* @__PURE__ */ __name(() => Xe, "__wbg_node_02999533c4ea02e3"), __wbg_now_2c95c9de01293173: /* @__PURE__ */ __name(() => Ke, "__wbg_now_2c95c9de01293173"), __wbg_performance_7a3ffd0b17f663ad: /* @__PURE__ */ __name(() => Qe, "__wbg_performance_7a3ffd0b17f663ad"), __wbg_prepare_d456fe7849727a77: /* @__PURE__ */ __name(() => Ye, "__wbg_prepare_d456fe7849727a77"), __wbg_process_5c1d670bc53614b8: /* @__PURE__ */ __name(() => Ze, "__wbg_process_5c1d670bc53614b8"), __wbg_push_737cfc8c1432c2c6: /* @__PURE__ */ __name(() => tn, "__wbg_push_737cfc8c1432c2c6"), __wbg_queueMicrotask_97d92b4fcc8a61c5: /* @__PURE__ */ __name(() => en, "__wbg_queueMicrotask_97d92b4fcc8a61c5"), __wbg_queueMicrotask_d3219def82552485: /* @__PURE__ */ __name(() => nn, "__wbg_queueMicrotask_d3219def82552485"), __wbg_randomFillSync_ab2cfe79ebbf2740: /* @__PURE__ */ __name(() => rn, "__wbg_randomFillSync_ab2cfe79ebbf2740"), __wbg_read_a2434af1186cb56c: /* @__PURE__ */ __name(() => _n, "__wbg_read_a2434af1186cb56c"), __wbg_redirect_14b0c8193458f8c3: /* @__PURE__ */ __name(() => on, "__wbg_redirect_14b0c8193458f8c3"), __wbg_releaseLock_091899af97991d2e: /* @__PURE__ */ __name(() => cn, "__wbg_releaseLock_091899af97991d2e"), __wbg_removeEventListener_056dfe8c3d6c58f9: /* @__PURE__ */ __name(() => sn, "__wbg_removeEventListener_056dfe8c3d6c58f9"), __wbg_require_79b1e9274cde3c87: /* @__PURE__ */ __name(() => un, "__wbg_require_79b1e9274cde3c87"), __wbg_resolve_4851785c9c5f573d: /* @__PURE__ */ __name(() => fn, "__wbg_resolve_4851785c9c5f573d"), __wbg_respond_1f279fa9f8edcb1c: /* @__PURE__ */ __name(() => an, "__wbg_respond_1f279fa9f8edcb1c"), __wbg_results_91d2999b650bcb82: /* @__PURE__ */ __name(() => bn, "__wbg_results_91d2999b650bcb82"), __wbg_run_aafb580d55f40d37: /* @__PURE__ */ __name(() => gn, "__wbg_run_aafb580d55f40d37"), __wbg_scheduledTime_ac17b6dc263066f8: /* @__PURE__ */ __name(() => dn, "__wbg_scheduledTime_ac17b6dc263066f8"), __wbg_send_0293179ba074ffb4: /* @__PURE__ */ __name(() => wn, "__wbg_send_0293179ba074ffb4"), __wbg_setTimeout_2e707715f8cc9497: /* @__PURE__ */ __name(() => pn, "__wbg_setTimeout_2e707715f8cc9497"), __wbg_set_11cd83f45504cedf: /* @__PURE__ */ __name(() => ln, "__wbg_set_11cd83f45504cedf"), __wbg_set_37837023f3d740e8: /* @__PURE__ */ __name(() => xn, "__wbg_set_37837023f3d740e8"), __wbg_set_3807d5f0bfc24aa7: /* @__PURE__ */ __name(() => mn, "__wbg_set_3807d5f0bfc24aa7"), __wbg_set_65595bdd868b3009: /* @__PURE__ */ __name(() => hn, "__wbg_set_65595bdd868b3009"), __wbg_set_8fc6bf8a5b1071d1: /* @__PURE__ */ __name(() => yn, "__wbg_set_8fc6bf8a5b1071d1"), __wbg_set_bb8cecf6a62b9f46: /* @__PURE__ */ __name(() => Rn, "__wbg_set_bb8cecf6a62b9f46"), __wbg_set_wasm: /* @__PURE__ */ __name(() => O, "__wbg_set_wasm"), __wbg_setbody_5923b78a95eedf29: /* @__PURE__ */ __name(() => Fn, "__wbg_setbody_5923b78a95eedf29"), __wbg_setcredentials_c3a22f1cd105a2c6: /* @__PURE__ */ __name(() => Sn, "__wbg_setcredentials_c3a22f1cd105a2c6"), __wbg_setheaders_3b47c898e8de6d44: /* @__PURE__ */ __name(() => kn, "__wbg_setheaders_3b47c898e8de6d44"), __wbg_setheaders_834c0bdb6a8949ad: /* @__PURE__ */ __name(() => In, "__wbg_setheaders_834c0bdb6a8949ad"), __wbg_sethighwatermark_793c99c89830c8e9: /* @__PURE__ */ __name(() => Tn, "__wbg_sethighwatermark_793c99c89830c8e9"), __wbg_setmethod_3c5280fe5d890842: /* @__PURE__ */ __name(() => En, "__wbg_setmethod_3c5280fe5d890842"), __wbg_setmode_5dc300b865044b65: /* @__PURE__ */ __name(() => jn, "__wbg_setmode_5dc300b865044b65"), __wbg_setredirect_40e6a7f717a2f86a: /* @__PURE__ */ __name(() => zn, "__wbg_setredirect_40e6a7f717a2f86a"), __wbg_setsignal_75b21ef3a81de905: /* @__PURE__ */ __name(() => On, "__wbg_setsignal_75b21ef3a81de905"), __wbg_setstatus_51b4fc011091cbb3: /* @__PURE__ */ __name(() => qn, "__wbg_setstatus_51b4fc011091cbb3"), __wbg_signal_02f4435f82019061: /* @__PURE__ */ __name(() => Mn, "__wbg_signal_02f4435f82019061"), __wbg_signal_aaf9ad74119f20a4: /* @__PURE__ */ __name(() => An, "__wbg_signal_aaf9ad74119f20a4"), __wbg_stack_0ed75d68575b0f3c: /* @__PURE__ */ __name(() => Ln, "__wbg_stack_0ed75d68575b0f3c"), __wbg_static_accessor_GLOBAL_88a902d13a557d07: /* @__PURE__ */ __name(() => Wn, "__wbg_static_accessor_GLOBAL_88a902d13a557d07"), __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0: /* @__PURE__ */ __name(() => vn, "__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0"), __wbg_static_accessor_SELF_37c5d418e4bf5819: /* @__PURE__ */ __name(() => Cn, "__wbg_static_accessor_SELF_37c5d418e4bf5819"), __wbg_static_accessor_WINDOW_5de37043a91a9c40: /* @__PURE__ */ __name(() => Un, "__wbg_static_accessor_WINDOW_5de37043a91a9c40"), __wbg_status_f6360336ca686bf0: /* @__PURE__ */ __name(() => Dn, "__wbg_status_f6360336ca686bf0"), __wbg_stringify_f7ed6987935b4a24: /* @__PURE__ */ __name(() => Bn, "__wbg_stringify_f7ed6987935b4a24"), __wbg_subarray_aa9065fa9dc5df96: /* @__PURE__ */ __name(() => Nn, "__wbg_subarray_aa9065fa9dc5df96"), __wbg_then_44b73946d2fb3e7d: /* @__PURE__ */ __name(() => Pn, "__wbg_then_44b73946d2fb3e7d"), __wbg_then_48b406749878a531: /* @__PURE__ */ __name(() => Vn, "__wbg_then_48b406749878a531"), __wbg_toString_c813bbd34d063839: /* @__PURE__ */ __name(() => $n, "__wbg_toString_c813bbd34d063839"), __wbg_url_8f9653b899456042: /* @__PURE__ */ __name(() => Jn, "__wbg_url_8f9653b899456042"), __wbg_url_ae10c34ca209681d: /* @__PURE__ */ __name(() => Gn, "__wbg_url_ae10c34ca209681d"), __wbg_value_cd1ffa7b1ab794f1: /* @__PURE__ */ __name(() => Hn, "__wbg_value_cd1ffa7b1ab794f1"), __wbg_versions_c71aa1626a93e0a1: /* @__PURE__ */ __name(() => Xn, "__wbg_versions_c71aa1626a93e0a1"), __wbg_view_fd8a56e8983f448d: /* @__PURE__ */ __name(() => Kn, "__wbg_view_fd8a56e8983f448d"), __wbg_webSocket_4a16d2250705e19d: /* @__PURE__ */ __name(() => Qn, "__wbg_webSocket_4a16d2250705e19d"), __wbindgen_as_number: /* @__PURE__ */ __name(() => Yn, "__wbindgen_as_number"), __wbindgen_bigint_from_u64: /* @__PURE__ */ __name(() => Zn, "__wbindgen_bigint_from_u64"), __wbindgen_bigint_get_as_i64: /* @__PURE__ */ __name(() => tr, "__wbindgen_bigint_get_as_i64"), __wbindgen_boolean_get: /* @__PURE__ */ __name(() => er, "__wbindgen_boolean_get"), __wbindgen_cb_drop: /* @__PURE__ */ __name(() => nr, "__wbindgen_cb_drop"), __wbindgen_closure_wrapper2902: /* @__PURE__ */ __name(() => rr, "__wbindgen_closure_wrapper2902"), __wbindgen_closure_wrapper2904: /* @__PURE__ */ __name(() => _r, "__wbindgen_closure_wrapper2904"), __wbindgen_closure_wrapper2906: /* @__PURE__ */ __name(() => or, "__wbindgen_closure_wrapper2906"), __wbindgen_closure_wrapper5291: /* @__PURE__ */ __name(() => cr, "__wbindgen_closure_wrapper5291"), __wbindgen_closure_wrapper5320: /* @__PURE__ */ __name(() => ir, "__wbindgen_closure_wrapper5320"), __wbindgen_debug_string: /* @__PURE__ */ __name(() => sr, "__wbindgen_debug_string"), __wbindgen_error_new: /* @__PURE__ */ __name(() => ur, "__wbindgen_error_new"), __wbindgen_in: /* @__PURE__ */ __name(() => fr, "__wbindgen_in"), __wbindgen_init_externref_table: /* @__PURE__ */ __name(() => ar, "__wbindgen_init_externref_table"), __wbindgen_is_bigint: /* @__PURE__ */ __name(() => br, "__wbindgen_is_bigint"), __wbindgen_is_function: /* @__PURE__ */ __name(() => gr, "__wbindgen_is_function"), __wbindgen_is_null: /* @__PURE__ */ __name(() => dr, "__wbindgen_is_null"), __wbindgen_is_object: /* @__PURE__ */ __name(() => wr, "__wbindgen_is_object"), __wbindgen_is_string: /* @__PURE__ */ __name(() => pr, "__wbindgen_is_string"), __wbindgen_is_undefined: /* @__PURE__ */ __name(() => lr, "__wbindgen_is_undefined"), __wbindgen_jsval_eq: /* @__PURE__ */ __name(() => xr, "__wbindgen_jsval_eq"), __wbindgen_jsval_loose_eq: /* @__PURE__ */ __name(() => mr, "__wbindgen_jsval_loose_eq"), __wbindgen_memory: /* @__PURE__ */ __name(() => hr, "__wbindgen_memory"), __wbindgen_number_get: /* @__PURE__ */ __name(() => yr, "__wbindgen_number_get"), __wbindgen_number_new: /* @__PURE__ */ __name(() => Rr, "__wbindgen_number_new"), __wbindgen_string_get: /* @__PURE__ */ __name(() => Fr, "__wbindgen_string_get"), __wbindgen_string_new: /* @__PURE__ */ __name(() => Sr, "__wbindgen_string_new"), __wbindgen_throw: /* @__PURE__ */ __name(() => kr, "__wbindgen_throw"), fetch: /* @__PURE__ */ __name(() => M, "fetch"), scheduled: /* @__PURE__ */ __name(() => q, "scheduled") });
var _;
function O(t2) {
  _ = t2;
}
__name(O, "O");
var d = 0;
var h = null;
function y() {
  return (h === null || h.byteLength === 0) && (h = new Uint8Array(_.memory.buffer)), h;
}
__name(y, "y");
var P = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var R = new P("utf-8");
var V = typeof R.encodeInto == "function" ? function(t2, e) {
  return R.encodeInto(t2, e);
} : function(t2, e) {
  let n = R.encode(t2);
  return e.set(n), { read: t2.length, written: n.length };
};
function w(t2, e, n) {
  if (n === void 0) {
    let a = R.encode(t2), x = e(a.length, 1) >>> 0;
    return y().subarray(x, x + a.length).set(a), d = a.length, x;
  }
  let r = t2.length, o = e(r, 1) >>> 0, b = y(), u = 0;
  for (; u < r; u++) {
    let a = t2.charCodeAt(u);
    if (a > 127) break;
    b[o + u] = a;
  }
  if (u !== r) {
    u !== 0 && (t2 = t2.slice(u)), o = n(o, r, r = u + t2.length * 3, 1) >>> 0;
    let a = y().subarray(o + u, o + r), x = V(t2, a);
    u += x.written, o = n(o, r, u, 1) >>> 0;
  }
  return d = u, o;
}
__name(w, "w");
var l = null;
function i() {
  return (l === null || l.buffer.detached === true || l.buffer.detached === void 0 && l.buffer !== _.memory.buffer) && (l = new DataView(_.memory.buffer)), l;
}
__name(i, "i");
function g(t2) {
  let e = _.__externref_table_alloc();
  return _.__wbindgen_export_4.set(e, t2), e;
}
__name(g, "g");
function c(t2, e) {
  try {
    return t2.apply(this, e);
  } catch (n) {
    let r = g(n);
    _.__wbindgen_exn_store(r);
  }
}
__name(c, "c");
var $ = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var C = new $("utf-8", { ignoreBOM: true, fatal: true });
C.decode();
function f(t2, e) {
  return t2 = t2 >>> 0, C.decode(y().subarray(t2, t2 + e));
}
__name(f, "f");
function s(t2) {
  return t2 == null;
}
__name(s, "s");
function J(t2, e) {
  let n = e(t2.length * 4, 4) >>> 0;
  for (let r = 0; r < t2.length; r++) {
    let o = g(t2[r]);
    i().setUint32(n + 4 * r, o, true);
  }
  return d = t2.length, n;
}
__name(J, "J");
var L = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t2) => {
  _.__wbindgen_export_6.get(t2.dtor)(t2.a, t2.b);
});
function m(t2, e, n, r) {
  let o = { a: t2, b: e, cnt: 1, dtor: n }, b = /* @__PURE__ */ __name((...u) => {
    o.cnt++;
    let a = o.a;
    o.a = 0;
    try {
      return r(a, o.b, ...u);
    } finally {
      --o.cnt === 0 ? (_.__wbindgen_export_6.get(o.dtor)(a, o.b), L.unregister(o)) : o.a = a;
    }
  }, "b");
  return b.original = o, L.register(b, o, o), b;
}
__name(m, "m");
function k(t2) {
  let e = typeof t2;
  if (e == "number" || e == "boolean" || t2 == null) return `${t2}`;
  if (e == "string") return `"${t2}"`;
  if (e == "symbol") {
    let o = t2.description;
    return o == null ? "Symbol" : `Symbol(${o})`;
  }
  if (e == "function") {
    let o = t2.name;
    return typeof o == "string" && o.length > 0 ? `Function(${o})` : "Function";
  }
  if (Array.isArray(t2)) {
    let o = t2.length, b = "[";
    o > 0 && (b += k(t2[0]));
    for (let u = 1; u < o; u++) b += ", " + k(t2[u]);
    return b += "]", b;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(t2)), r;
  if (n && n.length > 1) r = n[1];
  else return toString.call(t2);
  if (r == "Object") try {
    return "Object(" + JSON.stringify(t2) + ")";
  } catch {
    return "Object";
  }
  return t2 instanceof Error ? `${t2.name}: ${t2.message}
${t2.stack}` : r;
}
__name(k, "k");
function q(t2, e, n) {
  return _.scheduled(t2, e, n);
}
__name(q, "q");
function M(t2, e, n) {
  return _.fetch(t2, e, n);
}
__name(M, "M");
function A(t2, e, n) {
  _.closure980_externref_shim(t2, e, n);
}
__name(A, "A");
function G(t2, e) {
  _._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hdcbec2fea623b47f(t2, e);
}
__name(G, "G");
function H(t2, e, n) {
  _.closure1966_externref_shim(t2, e, n);
}
__name(H, "H");
function X(t2, e, n, r) {
  _.closure1988_externref_shim(t2, e, n, r);
}
__name(X, "X");
var K = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var Q = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var Y = ["bytes"];
var Z = ["omit", "same-origin", "include"];
var tt = ["same-origin", "no-cors", "cors", "navigate"];
var U = ["follow", "error", "manual"];
var et = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t2) => _.__wbg_intounderlyingbytesource_free(t2 >>> 0, 1));
var I = class {
  static {
    __name(this, "I");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, et.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_intounderlyingbytesource_free(e, 0);
  }
  get type() {
    let e = _.intounderlyingbytesource_type(this.__wbg_ptr);
    return Y[e];
  }
  get autoAllocateChunkSize() {
    return _.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(e) {
    _.intounderlyingbytesource_start(this.__wbg_ptr, e);
  }
  pull(e) {
    return _.intounderlyingbytesource_pull(this.__wbg_ptr, e);
  }
  cancel() {
    let e = this.__destroy_into_raw();
    _.intounderlyingbytesource_cancel(e);
  }
};
var nt = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t2) => _.__wbg_intounderlyingsink_free(t2 >>> 0, 1));
var T = class {
  static {
    __name(this, "T");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, nt.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_intounderlyingsink_free(e, 0);
  }
  write(e) {
    return _.intounderlyingsink_write(this.__wbg_ptr, e);
  }
  close() {
    let e = this.__destroy_into_raw();
    return _.intounderlyingsink_close(e);
  }
  abort(e) {
    let n = this.__destroy_into_raw();
    return _.intounderlyingsink_abort(n, e);
  }
};
var W = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t2) => _.__wbg_intounderlyingsource_free(t2 >>> 0, 1));
var F = class t {
  static {
    __name(this, "t");
  }
  static __wrap(e) {
    e = e >>> 0;
    let n = Object.create(t.prototype);
    return n.__wbg_ptr = e, W.register(n, n.__wbg_ptr, n), n;
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, W.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_intounderlyingsource_free(e, 0);
  }
  pull(e) {
    return _.intounderlyingsource_pull(this.__wbg_ptr, e);
  }
  cancel() {
    let e = this.__destroy_into_raw();
    _.intounderlyingsource_cancel(e);
  }
};
var rt = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t2) => _.__wbg_minifyconfig_free(t2 >>> 0, 1));
var E = class {
  static {
    __name(this, "E");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, rt.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_minifyconfig_free(e, 0);
  }
  get js() {
    return _.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(e) {
    _.__wbg_set_minifyconfig_js(this.__wbg_ptr, e);
  }
  get html() {
    return _.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(e) {
    _.__wbg_set_minifyconfig_html(this.__wbg_ptr, e);
  }
  get css() {
    return _.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(e) {
    _.__wbg_set_minifyconfig_css(this.__wbg_ptr, e);
  }
};
var v = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t2) => _.__wbg_msgbroker_free(t2 >>> 0, 1));
var j = class {
  static {
    __name(this, "j");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, v.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_msgbroker_free(e, 0);
  }
  constructor(e, n) {
    let r = _.msgbroker__new(e, n);
    return this.__wbg_ptr = r >>> 0, v.register(this, this.__wbg_ptr, this), this;
  }
  webSocketMessage(e, n) {
    return _.msgbroker_webSocketMessage(this.__wbg_ptr, e, n);
  }
  fetch(e) {
    return _.msgbroker_fetch(this.__wbg_ptr, e);
  }
};
var _t = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t2) => _.__wbg_r2range_free(t2 >>> 0, 1));
var z = class {
  static {
    __name(this, "z");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, _t.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_r2range_free(e, 0);
  }
  get offset() {
    let e = _.__wbg_get_r2range_offset(this.__wbg_ptr);
    return e[0] === 0 ? void 0 : e[1];
  }
  set offset(e) {
    _.__wbg_set_r2range_offset(this.__wbg_ptr, !s(e), s(e) ? 0 : e);
  }
  get length() {
    let e = _.__wbg_get_r2range_length(this.__wbg_ptr);
    return e[0] === 0 ? void 0 : e[1];
  }
  set length(e) {
    _.__wbg_set_r2range_length(this.__wbg_ptr, !s(e), s(e) ? 0 : e);
  }
  get suffix() {
    let e = _.__wbg_get_r2range_suffix(this.__wbg_ptr);
    return e[0] === 0 ? void 0 : e[1];
  }
  set suffix(e) {
    _.__wbg_set_r2range_suffix(this.__wbg_ptr, !s(e), s(e) ? 0 : e);
  }
};
function ot(t2, e) {
  let n = String(e), r = w(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = d;
  i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
}
__name(ot, "ot");
function ct(t2, e) {
  t2.abort(e);
}
__name(ct, "ct");
function it(t2) {
  t2.abort();
}
__name(it, "it");
function st() {
  return c(function(t2, e) {
    t2.acceptWebSocket(e);
  }, arguments);
}
__name(st, "st");
function ut() {
  return c(function(t2) {
    t2.accept();
  }, arguments);
}
__name(ut, "ut");
function ft() {
  return c(function(t2, e, n, r) {
    t2.addEventListener(f(e, n), r);
  }, arguments);
}
__name(ft, "ft");
function at() {
  return c(function(t2) {
    return t2.all();
  }, arguments);
}
__name(at, "at");
function bt() {
  return c(function(t2, e, n, r, o) {
    t2.append(f(e, n), f(r, o));
  }, arguments);
}
__name(bt, "bt");
function gt() {
  return c(function(t2) {
    return t2.arrayBuffer();
  }, arguments);
}
__name(gt, "gt");
function dt() {
  return c(function(t2, e) {
    return t2.bind(...e);
  }, arguments);
}
__name(dt, "dt");
function wt(t2) {
  let e = t2.body;
  return s(e) ? 0 : g(e);
}
__name(wt, "wt");
function pt(t2) {
  let e = t2.body;
  return s(e) ? 0 : g(e);
}
__name(pt, "pt");
function lt(t2) {
  return t2.buffer;
}
__name(lt, "lt");
function xt(t2) {
  return t2.buffer;
}
__name(xt, "xt");
function mt(t2) {
  let e = t2.byobRequest;
  return s(e) ? 0 : g(e);
}
__name(mt, "mt");
function ht(t2) {
  return t2.byteLength;
}
__name(ht, "ht");
function yt(t2) {
  return t2.byteOffset;
}
__name(yt, "yt");
function Rt() {
  return c(function(t2, e) {
    return t2.call(e);
  }, arguments);
}
__name(Rt, "Rt");
function Ft() {
  return c(function(t2, e, n) {
    return t2.call(e, n);
  }, arguments);
}
__name(Ft, "Ft");
function St() {
  return c(function(t2, e, n, r) {
    return t2.call(e, n, r);
  }, arguments);
}
__name(St, "St");
function kt() {
  return c(function(t2, e, n, r, o) {
    return t2.call(e, n, r, o);
  }, arguments);
}
__name(kt, "kt");
function It(t2) {
  return t2.cancel();
}
__name(It, "It");
function Tt(t2, e) {
  return t2.catch(e);
}
__name(Tt, "Tt");
function Et(t2) {
  return t2.cause;
}
__name(Et, "Et");
function jt() {
  return c(function(t2) {
    let e = t2.cf;
    return s(e) ? 0 : g(e);
  }, arguments);
}
__name(jt, "jt");
function zt() {
  return c(function(t2) {
    let e = t2.cf;
    return s(e) ? 0 : g(e);
  }, arguments);
}
__name(zt, "zt");
function Ot(t2) {
  return clearTimeout(t2);
}
__name(Ot, "Ot");
function qt() {
  return c(function(t2) {
    t2.close();
  }, arguments);
}
__name(qt, "qt");
function Mt() {
  return c(function(t2) {
    t2.close();
  }, arguments);
}
__name(Mt, "Mt");
function At() {
  return c(function(t2) {
    t2.close();
  }, arguments);
}
__name(At, "At");
function Lt(t2) {
  return t2.constructor;
}
__name(Lt, "Lt");
function Wt() {
  return c(function(t2, e) {
    let n = e.cron, r = w(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = d;
    i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
  }, arguments);
}
__name(Wt, "Wt");
function vt(t2) {
  return t2.crypto;
}
__name(vt, "vt");
function Ct(t2) {
  return t2.data;
}
__name(Ct, "Ct");
function Ut(t2) {
  console.debug(t2);
}
__name(Ut, "Ut");
function Dt(t2) {
  return t2.done;
}
__name(Dt, "Dt");
function Bt() {
  return c(function(t2, e) {
    t2.enqueue(e);
  }, arguments);
}
__name(Bt, "Bt");
function Nt(t2) {
  return t2.entries();
}
__name(Nt, "Nt");
function Pt(t2) {
  return t2.error;
}
__name(Pt, "Pt");
function Vt(t2) {
  console.error(t2);
}
__name(Vt, "Vt");
function $t(t2, e) {
  let n, r;
  try {
    n = t2, r = e, console.error(f(t2, e));
  } finally {
    _.__wbindgen_free(n, r, 1);
  }
}
__name($t, "$t");
function Jt(t2, e) {
  return t2.fetch(e);
}
__name(Jt, "Jt");
function Gt() {
  return c(function(t2, e) {
    return t2.fetch(e);
  }, arguments);
}
__name(Gt, "Gt");
function Ht(t2) {
  return fetch(t2);
}
__name(Ht, "Ht");
function Xt(t2) {
  return fetch(t2);
}
__name(Xt, "Xt");
function Kt() {
  return c(function(t2, e, n) {
    return t2.first(e === 0 ? void 0 : f(e, n));
  }, arguments);
}
__name(Kt, "Kt");
function Qt() {
  return c(function(t2, e) {
    t2.getRandomValues(e);
  }, arguments);
}
__name(Qt, "Qt");
function Yt() {
  return c(function(t2) {
    return t2.getReader();
  }, arguments);
}
__name(Yt, "Yt");
function Zt(t2) {
  return t2.getTime();
}
__name(Zt, "Zt");
function te() {
  return c(function(t2, e) {
    let n = e.getWebSockets(), r = J(n, _.__wbindgen_malloc), o = d;
    i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
  }, arguments);
}
__name(te, "te");
function ee() {
  return c(function(t2, e) {
    return Reflect.get(t2, e);
  }, arguments);
}
__name(ee, "ee");
function ne() {
  return c(function(t2, e) {
    return Reflect.get(t2, e >>> 0);
  }, arguments);
}
__name(ne, "ne");
function re() {
  return c(function(t2, e) {
    return t2.get(e);
  }, arguments);
}
__name(re, "re");
function _e(t2, e) {
  return t2[e >>> 0];
}
__name(_e, "_e");
function oe(t2) {
  let e = t2.done;
  return s(e) ? 16777215 : e ? 1 : 0;
}
__name(oe, "oe");
function ce(t2) {
  return t2.value;
}
__name(ce, "ce");
function ie(t2, e) {
  return t2[e];
}
__name(ie, "ie");
function se(t2, e) {
  return t2[e];
}
__name(se, "se");
function ue() {
  return c(function(t2, e) {
    return Reflect.has(t2, e);
  }, arguments);
}
__name(ue, "ue");
function fe(t2) {
  return t2.headers;
}
__name(fe, "fe");
function ae(t2) {
  return t2.headers;
}
__name(ae, "ae");
function be() {
  return c(function(t2, e) {
    let n = e.httpProtocol, r = w(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = d;
    i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
  }, arguments);
}
__name(be, "be");
function ge() {
  return c(function(t2, e, n) {
    return t2.idFromName(f(e, n));
  }, arguments);
}
__name(ge, "ge");
function de(t2) {
  let e;
  try {
    e = t2 instanceof ArrayBuffer;
  } catch {
    e = false;
  }
  return e;
}
__name(de, "de");
function we(t2) {
  let e;
  try {
    e = t2 instanceof Error;
  } catch {
    e = false;
  }
  return e;
}
__name(we, "we");
function pe(t2) {
  let e;
  try {
    e = t2 instanceof Response;
  } catch {
    e = false;
  }
  return e;
}
__name(pe, "pe");
function le(t2) {
  let e;
  try {
    e = t2 instanceof Uint8Array;
  } catch {
    e = false;
  }
  return e;
}
__name(le, "le");
function xe(t2) {
  return Number.isSafeInteger(t2);
}
__name(xe, "xe");
function me() {
  return Symbol.iterator;
}
__name(me, "me");
function he() {
  return c(function(t2) {
    return t2.json();
  }, arguments);
}
__name(he, "he");
function ye(t2) {
  return t2.length;
}
__name(ye, "ye");
function Re(t2) {
  return t2.length;
}
__name(Re, "Re");
function Fe(t2) {
  console.log(t2);
}
__name(Fe, "Fe");
function Se(t2) {
  return t2.message;
}
__name(Se, "Se");
function ke(t2, e) {
  let n = e.method, r = w(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = d;
  i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
}
__name(ke, "ke");
function Ie(t2) {
  return t2.msCrypto;
}
__name(Ie, "Ie");
function Te(t2) {
  return t2.name;
}
__name(Te, "Te");
function Ee() {
  return /* @__PURE__ */ new Date();
}
__name(Ee, "Ee");
function je() {
  return c(function() {
    return new Headers();
  }, arguments);
}
__name(je, "je");
function ze(t2, e) {
  try {
    var n = { a: t2, b: e }, r = /* @__PURE__ */ __name((b, u) => {
      let a = n.a;
      n.a = 0;
      try {
        return X(a, n.b, b, u);
      } finally {
        n.a = a;
      }
    }, "r");
    return new Promise(r);
  } finally {
    n.a = n.b = 0;
  }
}
__name(ze, "ze");
function Oe() {
  return new Object();
}
__name(Oe, "Oe");
function qe() {
  return /* @__PURE__ */ new Map();
}
__name(qe, "qe");
function Me() {
  return new Array();
}
__name(Me, "Me");
function Ae() {
  return new Error();
}
__name(Ae, "Ae");
function Le(t2) {
  return new Uint8Array(t2);
}
__name(Le, "Le");
function We(t2, e) {
  return new Error(f(t2, e));
}
__name(We, "We");
function ve() {
  return c(function() {
    return new AbortController();
  }, arguments);
}
__name(ve, "ve");
function Ce() {
  return c(function() {
    return new WebSocketPair();
  }, arguments);
}
__name(Ce, "Ce");
function Ue(t2, e) {
  return new Function(f(t2, e));
}
__name(Ue, "Ue");
function De(t2, e, n) {
  return new Uint8Array(t2, e >>> 0, n >>> 0);
}
__name(De, "De");
function Be(t2, e) {
  return new ReadableStream(F.__wrap(t2), e);
}
__name(Be, "Be");
function Ne(t2) {
  return new Uint8Array(t2 >>> 0);
}
__name(Ne, "Ne");
function Pe() {
  return c(function(t2, e) {
    return new Response(t2, e);
  }, arguments);
}
__name(Pe, "Pe");
function Ve() {
  return c(function(t2, e) {
    return new Response(t2, e);
  }, arguments);
}
__name(Ve, "Ve");
function $e() {
  return c(function(t2, e, n) {
    return new Response(t2 === 0 ? void 0 : f(t2, e), n);
  }, arguments);
}
__name($e, "$e");
function Je() {
  return c(function(t2, e, n) {
    return new Request(f(t2, e), n);
  }, arguments);
}
__name(Je, "Je");
function Ge(t2) {
  return t2.next;
}
__name(Ge, "Ge");
function He() {
  return c(function(t2) {
    return t2.next();
  }, arguments);
}
__name(He, "He");
function Xe(t2) {
  return t2.node;
}
__name(Xe, "Xe");
function Ke(t2) {
  return t2.now();
}
__name(Ke, "Ke");
function Qe(t2) {
  return t2.performance;
}
__name(Qe, "Qe");
function Ye() {
  return c(function(t2, e, n) {
    return t2.prepare(f(e, n));
  }, arguments);
}
__name(Ye, "Ye");
function Ze(t2) {
  return t2.process;
}
__name(Ze, "Ze");
function tn(t2, e) {
  return t2.push(e);
}
__name(tn, "tn");
function en(t2) {
  queueMicrotask(t2);
}
__name(en, "en");
function nn(t2) {
  return t2.queueMicrotask;
}
__name(nn, "nn");
function rn() {
  return c(function(t2, e) {
    t2.randomFillSync(e);
  }, arguments);
}
__name(rn, "rn");
function _n(t2) {
  return t2.read();
}
__name(_n, "_n");
function on(t2) {
  let e = t2.redirect;
  return (U.indexOf(e) + 1 || 4) - 1;
}
__name(on, "on");
function cn(t2) {
  t2.releaseLock();
}
__name(cn, "cn");
function sn() {
  return c(function(t2, e, n, r) {
    t2.removeEventListener(f(e, n), r);
  }, arguments);
}
__name(sn, "sn");
function un() {
  return c(function() {
    return module.require;
  }, arguments);
}
__name(un, "un");
function fn(t2) {
  return Promise.resolve(t2);
}
__name(fn, "fn");
function an() {
  return c(function(t2, e) {
    t2.respond(e >>> 0);
  }, arguments);
}
__name(an, "an");
function bn() {
  return c(function(t2) {
    let e = t2.results;
    return s(e) ? 0 : g(e);
  }, arguments);
}
__name(bn, "bn");
function gn() {
  return c(function(t2) {
    return t2.run();
  }, arguments);
}
__name(gn, "gn");
function dn() {
  return c(function(t2) {
    return t2.scheduledTime;
  }, arguments);
}
__name(dn, "dn");
function wn() {
  return c(function(t2, e, n) {
    t2.send(f(e, n));
  }, arguments);
}
__name(wn, "wn");
function pn(t2, e) {
  return setTimeout(t2, e);
}
__name(pn, "pn");
function ln() {
  return c(function(t2, e, n, r, o) {
    t2.set(f(e, n), f(r, o));
  }, arguments);
}
__name(ln, "ln");
function xn(t2, e, n) {
  t2[e >>> 0] = n;
}
__name(xn, "xn");
function mn(t2, e, n) {
  t2[e] = n;
}
__name(mn, "mn");
function hn(t2, e, n) {
  t2.set(e, n >>> 0);
}
__name(hn, "hn");
function yn(t2, e, n) {
  return t2.set(e, n);
}
__name(yn, "yn");
function Rn() {
  return c(function(t2, e, n) {
    return Reflect.set(t2, e, n);
  }, arguments);
}
__name(Rn, "Rn");
function Fn(t2, e) {
  t2.body = e;
}
__name(Fn, "Fn");
function Sn(t2, e) {
  t2.credentials = Z[e];
}
__name(Sn, "Sn");
function kn(t2, e) {
  t2.headers = e;
}
__name(kn, "kn");
function In(t2, e) {
  t2.headers = e;
}
__name(In, "In");
function Tn(t2, e) {
  t2.highWaterMark = e;
}
__name(Tn, "Tn");
function En(t2, e, n) {
  t2.method = f(e, n);
}
__name(En, "En");
function jn(t2, e) {
  t2.mode = tt[e];
}
__name(jn, "jn");
function zn(t2, e) {
  t2.redirect = U[e];
}
__name(zn, "zn");
function On(t2, e) {
  t2.signal = e;
}
__name(On, "On");
function qn(t2, e) {
  t2.status = e;
}
__name(qn, "qn");
function Mn(t2) {
  return t2.signal;
}
__name(Mn, "Mn");
function An(t2) {
  return t2.signal;
}
__name(An, "An");
function Ln(t2, e) {
  let n = e.stack, r = w(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = d;
  i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
}
__name(Ln, "Ln");
function Wn() {
  let t2 = typeof global > "u" ? null : global;
  return s(t2) ? 0 : g(t2);
}
__name(Wn, "Wn");
function vn() {
  let t2 = typeof globalThis > "u" ? null : globalThis;
  return s(t2) ? 0 : g(t2);
}
__name(vn, "vn");
function Cn() {
  let t2 = typeof self > "u" ? null : self;
  return s(t2) ? 0 : g(t2);
}
__name(Cn, "Cn");
function Un() {
  let t2 = typeof window > "u" ? null : window;
  return s(t2) ? 0 : g(t2);
}
__name(Un, "Un");
function Dn(t2) {
  return t2.status;
}
__name(Dn, "Dn");
function Bn() {
  return c(function(t2) {
    return JSON.stringify(t2);
  }, arguments);
}
__name(Bn, "Bn");
function Nn(t2, e, n) {
  return t2.subarray(e >>> 0, n >>> 0);
}
__name(Nn, "Nn");
function Pn(t2, e) {
  return t2.then(e);
}
__name(Pn, "Pn");
function Vn(t2, e, n) {
  return t2.then(e, n);
}
__name(Vn, "Vn");
function $n(t2) {
  return t2.toString();
}
__name($n, "$n");
function Jn(t2, e) {
  let n = e.url, r = w(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = d;
  i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
}
__name(Jn, "Jn");
function Gn(t2, e) {
  let n = e.url, r = w(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = d;
  i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
}
__name(Gn, "Gn");
function Hn(t2) {
  return t2.value;
}
__name(Hn, "Hn");
function Xn(t2) {
  return t2.versions;
}
__name(Xn, "Xn");
function Kn(t2) {
  let e = t2.view;
  return s(e) ? 0 : g(e);
}
__name(Kn, "Kn");
function Qn() {
  return c(function(t2) {
    let e = t2.webSocket;
    return s(e) ? 0 : g(e);
  }, arguments);
}
__name(Qn, "Qn");
function Yn(t2) {
  return +t2;
}
__name(Yn, "Yn");
function Zn(t2) {
  return BigInt.asUintN(64, t2);
}
__name(Zn, "Zn");
function tr(t2, e) {
  let n = e, r = typeof n == "bigint" ? n : void 0;
  i().setBigInt64(t2 + 8 * 1, s(r) ? BigInt(0) : r, true), i().setInt32(t2 + 4 * 0, !s(r), true);
}
__name(tr, "tr");
function er(t2) {
  let e = t2;
  return typeof e == "boolean" ? e ? 1 : 0 : 2;
}
__name(er, "er");
function nr(t2) {
  let e = t2.original;
  return e.cnt-- == 1 ? (e.a = 0, true) : false;
}
__name(nr, "nr");
function rr(t2, e, n) {
  return m(t2, e, 981, A);
}
__name(rr, "rr");
function _r(t2, e, n) {
  return m(t2, e, 981, A);
}
__name(_r, "_r");
function or(t2, e, n) {
  return m(t2, e, 981, A);
}
__name(or, "or");
function cr(t2, e, n) {
  return m(t2, e, 1962, G);
}
__name(cr, "cr");
function ir(t2, e, n) {
  return m(t2, e, 1967, H);
}
__name(ir, "ir");
function sr(t2, e) {
  let n = k(e), r = w(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = d;
  i().setInt32(t2 + 4 * 1, o, true), i().setInt32(t2 + 4 * 0, r, true);
}
__name(sr, "sr");
function ur(t2, e) {
  return new Error(f(t2, e));
}
__name(ur, "ur");
function fr(t2, e) {
  return t2 in e;
}
__name(fr, "fr");
function ar() {
  let t2 = _.__wbindgen_export_4, e = t2.grow(4);
  t2.set(0, void 0), t2.set(e + 0, void 0), t2.set(e + 1, null), t2.set(e + 2, true), t2.set(e + 3, false);
}
__name(ar, "ar");
function br(t2) {
  return typeof t2 == "bigint";
}
__name(br, "br");
function gr(t2) {
  return typeof t2 == "function";
}
__name(gr, "gr");
function dr(t2) {
  return t2 === null;
}
__name(dr, "dr");
function wr(t2) {
  let e = t2;
  return typeof e == "object" && e !== null;
}
__name(wr, "wr");
function pr(t2) {
  return typeof t2 == "string";
}
__name(pr, "pr");
function lr(t2) {
  return t2 === void 0;
}
__name(lr, "lr");
function xr(t2, e) {
  return t2 === e;
}
__name(xr, "xr");
function mr(t2, e) {
  return t2 == e;
}
__name(mr, "mr");
function hr() {
  return _.memory;
}
__name(hr, "hr");
function yr(t2, e) {
  let n = e, r = typeof n == "number" ? n : void 0;
  i().setFloat64(t2 + 8 * 1, s(r) ? 0 : r, true), i().setInt32(t2 + 4 * 0, !s(r), true);
}
__name(yr, "yr");
function Rr(t2) {
  return t2;
}
__name(Rr, "Rr");
function Fr(t2, e) {
  let n = e, r = typeof n == "string" ? n : void 0;
  var o = s(r) ? 0 : w(r, _.__wbindgen_malloc, _.__wbindgen_realloc), b = d;
  i().setInt32(t2 + 4 * 1, b, true), i().setInt32(t2 + 4 * 0, o, true);
}
__name(Fr, "Fr");
function Sr(t2, e) {
  return f(t2, e);
}
__name(Sr, "Sr");
function kr(t2, e) {
  throw new Error(f(t2, e));
}
__name(kr, "kr");
var D = new WebAssembly.Instance(Ir, { "./index_bg.js": p });
O(D.exports);
D.exports.__wbindgen_start?.();
var S = class extends Tr {
  static {
    __name(this, "S");
  }
  async fetch(e) {
    return await M(e, this.env, this.ctx);
  }
  async queue(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
  async scheduled(e) {
    return await q(e, this.env, this.ctx);
  }
};
var Er = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(p).map((t2) => {
  Er.includes(t2) | t2.startsWith("__") || (S.prototype[t2] = p[t2]);
});
var qr = S;
export {
  I as IntoUnderlyingByteSource,
  T as IntoUnderlyingSink,
  F as IntoUnderlyingSource,
  E as MinifyConfig,
  j as MsgBroker,
  K as PolishConfig,
  z as R2Range,
  Q as RequestRedirect,
  ot as __wbg_String_8f0eb39a4a4c2f66,
  ct as __wbg_abort_410ec47a64ac6117,
  it as __wbg_abort_775ef1d17fc65868,
  st as __wbg_acceptWebSocket_163b415f37889fa6,
  ut as __wbg_accept_2c43be5ee3ef3652,
  ft as __wbg_addEventListener_90e553fdce254421,
  at as __wbg_all_8de32f92c8bbd12f,
  bt as __wbg_append_8c7dd8d641a5f01b,
  gt as __wbg_arrayBuffer_d1b44c4390db422f,
  dt as __wbg_bind_93c078d60dfda7d4,
  wt as __wbg_body_018617e858cb7195,
  pt as __wbg_body_0b8fd1fe671660df,
  lt as __wbg_buffer_09165b52af8c5237,
  xt as __wbg_buffer_609cc3eee51ed158,
  mt as __wbg_byobRequest_77d9adf63337edfb,
  ht as __wbg_byteLength_e674b853d9c77e1d,
  yt as __wbg_byteOffset_fd862df290ef848d,
  Rt as __wbg_call_672a4d21634d4a24,
  Ft as __wbg_call_7cccdd69e0791ae2,
  St as __wbg_call_833bed5770ea2041,
  kt as __wbg_call_b8adc8b1d0a0d8eb,
  It as __wbg_cancel_8a308660caa6cadf,
  Tt as __wbg_catch_a6e601879b2610e9,
  Et as __wbg_cause_9940c4e8dfcd5129,
  jt as __wbg_cf_123509d53a2ea003,
  zt as __wbg_cf_abc51304c8a6868c,
  Ot as __wbg_clearTimeout_86721db0036bea98,
  qt as __wbg_close_2893b7d056a0627d,
  Mt as __wbg_close_304cc1fef3466669,
  At as __wbg_close_5ce03e29be453811,
  Lt as __wbg_constructor_9fd96f589d65d4e5,
  Wt as __wbg_cron_232bea08224e3e32,
  vt as __wbg_crypto_ed58b8e10a292839,
  Ct as __wbg_data_432d9c3df2630942,
  Ut as __wbg_debug_3cb59063b29f58c1,
  Dt as __wbg_done_769e5ede4b31c67b,
  Bt as __wbg_enqueue_bb16ba72f537dc9e,
  Nt as __wbg_entries_2a52db465d0421fb,
  Pt as __wbg_error_0554b4a81edb112e,
  Vt as __wbg_error_524f506f44df1645,
  $t as __wbg_error_7534b8e9a36f1ab4,
  Jt as __wbg_fetch_509096533071c657,
  Gt as __wbg_fetch_553598c1b6e65360,
  Ht as __wbg_fetch_72dfa99e9fd87dd7,
  Xt as __wbg_fetch_d36a73832f0a45e8,
  Kt as __wbg_first_b0c1bd826f2fcb2d,
  Qt as __wbg_getRandomValues_bcb4912f16000dc4,
  Yt as __wbg_getReader_48e00749fe3f6089,
  Zt as __wbg_getTime_46267b1c24877e30,
  te as __wbg_getWebSockets_f18105d7b88608ad,
  ee as __wbg_get_67b2ba62fc30de12,
  ne as __wbg_get_85c3d71662a108c8,
  re as __wbg_get_b0ab2e46022a2b56,
  _e as __wbg_get_b9b93047fe3cf45b,
  oe as __wbg_getdone_d47073731acd3e74,
  ce as __wbg_getvalue_009dcd63692bee1f,
  ie as __wbg_getwithrefkey_1dc361bd10053bfe,
  se as __wbg_getwithrefkey_6550b2c093d2eb18,
  ue as __wbg_has_a5ea9117f258a0ec,
  fe as __wbg_headers_7852a8ea641c1379,
  ae as __wbg_headers_9cb51cfd2ac780a4,
  be as __wbg_httpProtocol_4cc3ab4fde2ecf82,
  ge as __wbg_idFromName_7b7b4a116f3a6ed2,
  de as __wbg_instanceof_ArrayBuffer_e14585432e3737fc,
  we as __wbg_instanceof_Error_4d54113b22d20306,
  pe as __wbg_instanceof_Response_f2cc20d9f7dfd644,
  le as __wbg_instanceof_Uint8Array_17156bcf118086a9,
  xe as __wbg_isSafeInteger_343e2beeeece1bb0,
  me as __wbg_iterator_9a24c88df860dc65,
  he as __wbg_json_a00f187c0be01957,
  ye as __wbg_length_a446193dc22c12f8,
  Re as __wbg_length_e2d2a49132c1b256,
  Fe as __wbg_log_c222819a41e063d3,
  Se as __wbg_message_97a2af9b89d693a3,
  ke as __wbg_method_3dcc854b644c5a56,
  Ie as __wbg_msCrypto_0a36e2ec3a343d26,
  Te as __wbg_name_16617c8e9d4188ac,
  Ee as __wbg_new0_f788a2397c7ca929,
  je as __wbg_new_018dcc2d6c8c2f6a,
  ze as __wbg_new_23a2665fac83c611,
  Oe as __wbg_new_405e22f390576ce2,
  qe as __wbg_new_5e0be73521bc8c17,
  Me as __wbg_new_78feb108b6472713,
  Ae as __wbg_new_8a6f238a6ece86ea,
  Le as __wbg_new_a12002a7f91c75be,
  We as __wbg_new_c68d7209be747379,
  ve as __wbg_new_e25e5aab09ff45db,
  Ce as __wbg_new_f20415acc2b509a9,
  Ue as __wbg_newnoargs_105ed471475aaf50,
  De as __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a,
  Be as __wbg_newwithintounderlyingsource_b47f6a6a596a7f24,
  Ne as __wbg_newwithlength_a381634e90c276d4,
  Pe as __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1,
  Ve as __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e,
  $e as __wbg_newwithoptstrandinit_615a266ef226c260,
  Je as __wbg_newwithstrandinit_06c535e0a867c635,
  Ge as __wbg_next_25feadfc0913fea9,
  He as __wbg_next_6574e1a8a62d1055,
  Xe as __wbg_node_02999533c4ea02e3,
  Ke as __wbg_now_2c95c9de01293173,
  Qe as __wbg_performance_7a3ffd0b17f663ad,
  Ye as __wbg_prepare_d456fe7849727a77,
  Ze as __wbg_process_5c1d670bc53614b8,
  tn as __wbg_push_737cfc8c1432c2c6,
  en as __wbg_queueMicrotask_97d92b4fcc8a61c5,
  nn as __wbg_queueMicrotask_d3219def82552485,
  rn as __wbg_randomFillSync_ab2cfe79ebbf2740,
  _n as __wbg_read_a2434af1186cb56c,
  on as __wbg_redirect_14b0c8193458f8c3,
  cn as __wbg_releaseLock_091899af97991d2e,
  sn as __wbg_removeEventListener_056dfe8c3d6c58f9,
  un as __wbg_require_79b1e9274cde3c87,
  fn as __wbg_resolve_4851785c9c5f573d,
  an as __wbg_respond_1f279fa9f8edcb1c,
  bn as __wbg_results_91d2999b650bcb82,
  gn as __wbg_run_aafb580d55f40d37,
  dn as __wbg_scheduledTime_ac17b6dc263066f8,
  wn as __wbg_send_0293179ba074ffb4,
  pn as __wbg_setTimeout_2e707715f8cc9497,
  ln as __wbg_set_11cd83f45504cedf,
  xn as __wbg_set_37837023f3d740e8,
  mn as __wbg_set_3807d5f0bfc24aa7,
  hn as __wbg_set_65595bdd868b3009,
  yn as __wbg_set_8fc6bf8a5b1071d1,
  Rn as __wbg_set_bb8cecf6a62b9f46,
  O as __wbg_set_wasm,
  Fn as __wbg_setbody_5923b78a95eedf29,
  Sn as __wbg_setcredentials_c3a22f1cd105a2c6,
  kn as __wbg_setheaders_3b47c898e8de6d44,
  In as __wbg_setheaders_834c0bdb6a8949ad,
  Tn as __wbg_sethighwatermark_793c99c89830c8e9,
  En as __wbg_setmethod_3c5280fe5d890842,
  jn as __wbg_setmode_5dc300b865044b65,
  zn as __wbg_setredirect_40e6a7f717a2f86a,
  On as __wbg_setsignal_75b21ef3a81de905,
  qn as __wbg_setstatus_51b4fc011091cbb3,
  Mn as __wbg_signal_02f4435f82019061,
  An as __wbg_signal_aaf9ad74119f20a4,
  Ln as __wbg_stack_0ed75d68575b0f3c,
  Wn as __wbg_static_accessor_GLOBAL_88a902d13a557d07,
  vn as __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0,
  Cn as __wbg_static_accessor_SELF_37c5d418e4bf5819,
  Un as __wbg_static_accessor_WINDOW_5de37043a91a9c40,
  Dn as __wbg_status_f6360336ca686bf0,
  Bn as __wbg_stringify_f7ed6987935b4a24,
  Nn as __wbg_subarray_aa9065fa9dc5df96,
  Pn as __wbg_then_44b73946d2fb3e7d,
  Vn as __wbg_then_48b406749878a531,
  $n as __wbg_toString_c813bbd34d063839,
  Jn as __wbg_url_8f9653b899456042,
  Gn as __wbg_url_ae10c34ca209681d,
  Hn as __wbg_value_cd1ffa7b1ab794f1,
  Xn as __wbg_versions_c71aa1626a93e0a1,
  Kn as __wbg_view_fd8a56e8983f448d,
  Qn as __wbg_webSocket_4a16d2250705e19d,
  Yn as __wbindgen_as_number,
  Zn as __wbindgen_bigint_from_u64,
  tr as __wbindgen_bigint_get_as_i64,
  er as __wbindgen_boolean_get,
  nr as __wbindgen_cb_drop,
  rr as __wbindgen_closure_wrapper2902,
  _r as __wbindgen_closure_wrapper2904,
  or as __wbindgen_closure_wrapper2906,
  cr as __wbindgen_closure_wrapper5291,
  ir as __wbindgen_closure_wrapper5320,
  sr as __wbindgen_debug_string,
  ur as __wbindgen_error_new,
  fr as __wbindgen_in,
  ar as __wbindgen_init_externref_table,
  br as __wbindgen_is_bigint,
  gr as __wbindgen_is_function,
  dr as __wbindgen_is_null,
  wr as __wbindgen_is_object,
  pr as __wbindgen_is_string,
  lr as __wbindgen_is_undefined,
  xr as __wbindgen_jsval_eq,
  mr as __wbindgen_jsval_loose_eq,
  hr as __wbindgen_memory,
  yr as __wbindgen_number_get,
  Rr as __wbindgen_number_new,
  Fr as __wbindgen_string_get,
  Sr as __wbindgen_string_new,
  kr as __wbindgen_throw,
  qr as default,
  M as fetch,
  q as scheduled,
  Ir as wasmModule
};
//# sourceMappingURL=shim.js.map
