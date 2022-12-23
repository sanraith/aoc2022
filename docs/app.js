(()=>{"use strict";var e,n,_,r,t={982:(e,n,_)=>{_.d(n,{Z:()=>c});var r=_(81),t=_.n(r),a=_(645),o=_.n(a)()(t());o.push([e.id,":root {\r\n    --term-width: calc(16px * 90);\r\n    --term-height: calc(16px * 50);\r\n}\r\n\r\nbody {\r\n    background-color: #202020;\r\n    margin: 0;\r\n    height: 100%;\r\n    color: whitesmoke;\r\n    font-family: 'Courier New', Courier, monospace;\r\n}\r\n\r\n.center {\r\n    position: absolute;\r\n    top: 50%;\r\n    left: 50%;\r\n    transform: translateY(-50%) translateX(-50%);\r\n    width: var(--term-width);\r\n    height: var(--term-height);\r\n\r\n    border: 5px solid darkgreen;\r\n    border-radius: 5px;\r\n}\r\n\r\n.container {\r\n    position: relative;\r\n    width: var(--term-width);\r\n    height: var(--term-height);\r\n    background-color: black;\r\n}\r\n\r\n#canvas-placeholder {\r\n    position: absolute;\r\n    top: 0;\r\n    width: var(--term-width);\r\n    height: var(--term-height);\r\n    padding: 2em 2em;\r\n}\r\n\r\n#canvas-placeholder h1 {\r\n    text-align: center;\r\n    font-size: 32px;\r\n    font-weight: bold;\r\n}\r\n\r\n#canvas {\r\n    position: absolute;\r\n    top: 0;\r\n    display: block;\r\n    width: var(--term-width);\r\n    height: var(--term-height);\r\n}",""]);const c=o},645:e=>{e.exports=function(e){var n=[];return n.toString=function(){return this.map((function(n){var _="",r=void 0!==n[5];return n[4]&&(_+="@supports (".concat(n[4],") {")),n[2]&&(_+="@media ".concat(n[2]," {")),r&&(_+="@layer".concat(n[5].length>0?" ".concat(n[5]):""," {")),_+=e(n),r&&(_+="}"),n[2]&&(_+="}"),n[4]&&(_+="}"),_})).join("")},n.i=function(e,_,r,t,a){"string"==typeof e&&(e=[[null,e,void 0]]);var o={};if(r)for(var c=0;c<this.length;c++){var f=this[c][0];null!=f&&(o[f]=!0)}for(var b=0;b<e.length;b++){var i=[].concat(e[b]);r&&o[i[0]]||(void 0!==a&&(void 0===i[5]||(i[1]="@layer".concat(i[5].length>0?" ".concat(i[5]):""," {").concat(i[1],"}")),i[5]=a),_&&(i[2]?(i[1]="@media ".concat(i[2]," {").concat(i[1],"}"),i[2]=_):i[2]=_),t&&(i[4]?(i[1]="@supports (".concat(i[4],") {").concat(i[1],"}"),i[4]=t):i[4]="".concat(t)),n.push(i))}},n}},81:e=>{e.exports=function(e){return e[1]}},379:e=>{var n=[];function _(e){for(var _=-1,r=0;r<n.length;r++)if(n[r].identifier===e){_=r;break}return _}function r(e,r){for(var a={},o=[],c=0;c<e.length;c++){var f=e[c],b=r.base?f[0]+r.base:f[0],i=a[b]||0,d="".concat(b," ").concat(i);a[b]=i+1;var u=_(d),g={css:f[1],media:f[2],sourceMap:f[3],supports:f[4],layer:f[5]};if(-1!==u)n[u].references++,n[u].updater(g);else{var s=t(g,r);r.byIndex=c,n.splice(c,0,{identifier:d,updater:s,references:1})}o.push(d)}return o}function t(e,n){var _=n.domAPI(n);return _.update(e),function(n){if(n){if(n.css===e.css&&n.media===e.media&&n.sourceMap===e.sourceMap&&n.supports===e.supports&&n.layer===e.layer)return;_.update(e=n)}else _.remove()}}e.exports=function(e,t){var a=r(e=e||[],t=t||{});return function(e){e=e||[];for(var o=0;o<a.length;o++){var c=_(a[o]);n[c].references--}for(var f=r(e,t),b=0;b<a.length;b++){var i=_(a[b]);0===n[i].references&&(n[i].updater(),n.splice(i,1))}a=f}}},569:e=>{var n={};e.exports=function(e,_){var r=function(e){if(void 0===n[e]){var _=document.querySelector(e);if(window.HTMLIFrameElement&&_ instanceof window.HTMLIFrameElement)try{_=_.contentDocument.head}catch(e){_=null}n[e]=_}return n[e]}(e);if(!r)throw new Error("Couldn't find a style target. This probably means that the value for the 'insert' parameter is invalid.");r.appendChild(_)}},216:e=>{e.exports=function(e){var n=document.createElement("style");return e.setAttributes(n,e.attributes),e.insert(n,e.options),n}},565:(e,n,_)=>{e.exports=function(e){var n=_.nc;n&&e.setAttribute("nonce",n)}},795:e=>{e.exports=function(e){var n=e.insertStyleElement(e);return{update:function(_){!function(e,n,_){var r="";_.supports&&(r+="@supports (".concat(_.supports,") {")),_.media&&(r+="@media ".concat(_.media," {"));var t=void 0!==_.layer;t&&(r+="@layer".concat(_.layer.length>0?" ".concat(_.layer):""," {")),r+=_.css,t&&(r+="}"),_.media&&(r+="}"),_.supports&&(r+="}");var a=_.sourceMap;a&&"undefined"!=typeof btoa&&(r+="\n/*# sourceMappingURL=data:application/json;base64,".concat(btoa(unescape(encodeURIComponent(JSON.stringify(a))))," */")),n.styleTagTransform(r,e,n.options)}(n,e,_)},remove:function(){!function(e){if(null===e.parentNode)return!1;e.parentNode.removeChild(e)}(n)}}}},589:e=>{e.exports=function(e,n){if(n.styleSheet)n.styleSheet.cssText=e;else{for(;n.firstChild;)n.removeChild(n.firstChild);n.appendChild(document.createTextNode(e))}}},391:(e,n,_)=>{_.a(e,(async(e,r)=>{try{_.r(n),_.d(n,{__wbg_attachShader_90ad543fb1bccb18:()=>t.vK,__wbg_attachShader_f4d51147351a1906:()=>t.tX,__wbg_bindBuffer_66e359418f5c82d7:()=>t.Eo,__wbg_bindBuffer_8b5135aa633680f5:()=>t.kJ,__wbg_bindFramebuffer_080d0b0cf22e1645:()=>t.Y,__wbg_bindFramebuffer_5c01742edd5d843a:()=>t.O_,__wbg_bindTexture_6f1dec563e82e818:()=>t.sG,__wbg_bindTexture_ae9620ea4a6ffb97:()=>t.Y7,__wbg_bindVertexArrayOES_84540c072ea96b75:()=>t.XM,__wbg_bindVertexArray_9d12800e272184b0:()=>t.E7,__wbg_blendFunc_49ea28240d4c1084:()=>t.UO,__wbg_blendFunc_99b48b64bde98c6f:()=>t.Qf,__wbg_bufferData_8d206d7adf6751c0:()=>t.zi,__wbg_bufferData_a33528a74dd300f4:()=>t.G8,__wbg_buffer_3f3d764d4747d564:()=>t.jp,__wbg_call_168da88779e35f61:()=>t.VD,__wbg_call_97ae9d8645dc388b:()=>t.Ni,__wbg_charCode_b0f31612a52c2bff:()=>t.Dn,__wbg_clearColor_7489a3fbe484f2f1:()=>t.PX,__wbg_clearColor_bc89a6580c0498c3:()=>t.Mk,__wbg_clear_05614d3b84e96aae:()=>t.M$,__wbg_clear_576f67967748e95f:()=>t._n,__wbg_code_06787cd3c7a60600:()=>t.WM,__wbg_compileShader_22b038faa1f49857:()=>t.xJ,__wbg_compileShader_822f38928f6f2a08:()=>t.LD,__wbg_createBuffer_6e747d928c9ba46d:()=>t.cq,__wbg_createBuffer_a6cffb7f7d5b92a3:()=>t.Me,__wbg_createFramebuffer_9b5b0507480146cd:()=>t.QW,__wbg_createFramebuffer_d5f3985ce3652661:()=>t.T7,__wbg_createProgram_1c5f8dffd1066e71:()=>t.JH,__wbg_createProgram_dc6b23d3caa1d86e:()=>t.jb,__wbg_createShader_4017d9fbc36659af:()=>t.am,__wbg_createShader_46a66dce5a9e22d0:()=>t.G$,__wbg_createTexture_269f67d411bdc4dc:()=>t.Qy,__wbg_createTexture_4ce49e8a8c655124:()=>t.Il,__wbg_createVertexArrayOES_00a5c523e5b17eff:()=>t.rk,__wbg_createVertexArray_8467a75e68fec199:()=>t.lX,__wbg_crypto_e1d53a1d73fb10b8:()=>t.Nk,__wbg_disable_1659dc1efb5fb934:()=>t.Zs,__wbg_disable_6835d16c2cd3fa26:()=>t.or,__wbg_document_3ead31dbcad65886:()=>t.Nl,__wbg_drawArrays_c0dcb4151e0bf007:()=>t.qP,__wbg_drawArrays_d587302f7a868d91:()=>t.Oq,__wbg_drawElements_241caa588795bcb1:()=>t.JX,__wbg_drawElements_e09dbef58c8f099a:()=>t.X$,__wbg_enableVertexAttribArray_3d21f4936ad4a378:()=>t.rZ,__wbg_enableVertexAttribArray_a1ffc091f3999354:()=>t.jY,__wbg_enable_4791414dce6f602a:()=>t.cs,__wbg_enable_fc393941ac400f72:()=>t.RU,__wbg_error_f851667af71bcfc6:()=>t.iX,__wbg_framebufferTexture2D_499d1c21458d0113:()=>t.LQ,__wbg_framebufferTexture2D_4b810902dffa1ef3:()=>t.Oh,__wbg_getContext_4d5e97892c1b206a:()=>t.qh,__wbg_getElementById_3a708b83e4f034d7:()=>t.Z_,__wbg_getError_8de2be43ffb2c4e0:()=>t.CS,__wbg_getError_9ace44157772dd10:()=>t.ig,__wbg_getExtension_e7912bce04869d40:()=>t.KC,__wbg_getModifierState_135305ae40997dc7:()=>t.O6,__wbg_getProgramInfoLog_1e37a3d1d090ec1c:()=>t.pB,__wbg_getProgramInfoLog_e47d5073d57fb18d:()=>t.sb,__wbg_getProgramParameter_acf4ae158143e2b2:()=>t.Lh,__wbg_getProgramParameter_eaf768a9b399b7cf:()=>t.hO,__wbg_getRandomValues_805f1c3d65988a5a:()=>t.e,__wbg_getShaderInfoLog_451545b963646762:()=>t.If,__wbg_getShaderInfoLog_ec7e5b959e47645b:()=>t.Jn,__wbg_getShaderParameter_42a35b974329561c:()=>t.Mt,__wbg_getShaderParameter_6cd8c36fded266ea:()=>t.q,__wbg_getUniformLocation_0da0c93f626244a2:()=>t.SF,__wbg_getUniformLocation_8e9cc276a231ddcd:()=>t.w_,__wbg_get_765201544a2b6869:()=>t.bO,__wbg_globalThis_7f206bda628d5286:()=>t.$L,__wbg_global_ba75c50d1cf384f4:()=>t.wJ,__wbg_instanceof_HtmlCanvasElement_97761617af6ea089:()=>t.YL,__wbg_instanceof_WebGl2RenderingContext_fcfa91cd777063f3:()=>t.ir,__wbg_instanceof_Window_acc97ff9f5d2c7b4:()=>t.cE,__wbg_keyCode_72faed4278f77f2c:()=>t.XX,__wbg_length_9e1ae1900cb0fbd5:()=>t.bj,__wbg_linkProgram_25cda5f9318ea316:()=>t.n4,__wbg_linkProgram_c33885d9ea798810:()=>t.Oz,__wbg_log_b09521c515df0f23:()=>t.p1,__wbg_msCrypto_6e7d3e1f92610cbb:()=>t.cr,__wbg_new_8c3f0052272a457a:()=>t.lB,__wbg_new_abda76e883ba8a5f:()=>t.a2,__wbg_newnoargs_b5b063fc6c2f0376:()=>t.gW,__wbg_newwithbyteoffsetandlength_5540e144e9b8b907:()=>t.Zx,__wbg_newwithbyteoffsetandlength_698c5100ae9c3365:()=>t.H1,__wbg_newwithbyteoffsetandlength_7be13f49af2b2012:()=>t.HT,__wbg_newwithbyteoffsetandlength_890b478c8d7226ff:()=>t.EQ,__wbg_newwithbyteoffsetandlength_9cc9adccd861aa26:()=>t.Sr,__wbg_newwithbyteoffsetandlength_be22e5fcf4f69ab4:()=>t.Mv,__wbg_newwithbyteoffsetandlength_d9aa266703cb98be:()=>t.TY,__wbg_newwithlength_f5933855e4f48a19:()=>t.ib,__wbg_node_080f4b19d15bc1fe:()=>t.f5,__wbg_now_58886682b7e790d7:()=>t.RL,__wbg_now_8172cd917e5eda6b:()=>t.r4,__wbg_offsetX_8891849b36542d53:()=>t.Ab,__wbg_offsetY_1f52082687af467b:()=>t.Qc,__wbg_performance_de9825f9a8678574:()=>t.ac,__wbg_postMessage_8a8f33d997e17e7b:()=>t.uR,__wbg_postMessage_ef1cef7e229c46d7:()=>t.dC,__wbg_process_038c26bf42b093f8:()=>t.DI,__wbg_randomFillSync_6894564c2c334c42:()=>t.ex,__wbg_requestAnimationFrame_4181656476a7d86c:()=>t.$o,__wbg_require_78a3dcfbdba9cbce:()=>t.go,__wbg_self_6d479506f72c6a71:()=>t.yB,__wbg_set_83db9690f9353e79:()=>t.fP,__wbg_setheight_3eb8729b59493242:()=>t.bB,__wbg_setonkeydown_ddc0009c6d7693cd:()=>t.xH,__wbg_setonkeyup_4e1eff214c25854c:()=>t.Jx,__wbg_setonmousedown_8778ff85c1c9cd52:()=>t.sK,__wbg_setonmousemove_ef55e5efd369f524:()=>t.tf,__wbg_setonmouseup_39fd2509e8c154de:()=>t.$7,__wbg_setwidth_afb418d3fbf71ba7:()=>t.tI,__wbg_shaderSource_5111981e7afb61fb:()=>t.VI,__wbg_shaderSource_a0001b8eab5d44f4:()=>t.y3,__wbg_stack_658279fe44541cf6:()=>t.KM,__wbg_subarray_58ad4efbb5bcb886:()=>t.kH,__wbg_texImage2D_1bc6fe2370a72e1c:()=>t.fD,__wbg_texImage2D_5b25282e44d0e3fe:()=>t.gM,__wbg_texParameteri_1b210b807f1ea723:()=>t.cw,__wbg_texParameteri_21fd6b6b394882c9:()=>t.$d,__wbg_uniform1i_49986febd844f2c4:()=>t.JT,__wbg_uniform1i_50124a48de1da66b:()=>t.gP,__wbg_uniform3f_35a7a76696c08aa4:()=>t.$C,__wbg_uniform3f_d756c07788fa91da:()=>t.gj,__wbg_useProgram_156511a425feb519:()=>t.Vx,__wbg_useProgram_35a58ac1e0d9577b:()=>t.wk,__wbg_versions_ab37218d2f0b24a8:()=>t.QT,__wbg_vertexAttribPointer_3b06d737566f0745:()=>t.PP,__wbg_vertexAttribPointer_63d2aef49627302b:()=>t.ZR,__wbg_window_f2557cc78490aceb:()=>t.Sn,__wbindgen_boolean_get:()=>t.sZ,__wbindgen_cb_drop:()=>t.G6,__wbindgen_closure_wrapper2330:()=>t.yD,__wbindgen_closure_wrapper2332:()=>t.rw,__wbindgen_closure_wrapper359:()=>t.nh,__wbindgen_debug_string:()=>t.fY,__wbindgen_is_function:()=>t.o$,__wbindgen_is_object:()=>t.Wl,__wbindgen_is_string:()=>t.eY,__wbindgen_is_undefined:()=>t.XP,__wbindgen_memory:()=>t.oH,__wbindgen_number_get:()=>t.M1,__wbindgen_object_clone_ref:()=>t.m_,__wbindgen_object_drop_ref:()=>t.ug,__wbindgen_string_get:()=>t.qt,__wbindgen_string_new:()=>t.h4,__wbindgen_throw:()=>t.Or,main_wasm:()=>t.J5,on_worker_message:()=>t.hn,push_key_event:()=>t.em,set_scale:()=>t.iE,set_worker:()=>t.TL,worker_on_message:()=>t.xA,worker_set_global_scope:()=>t.bG});var t=_(686),a=e([t]);t=(a.then?(await a)():a)[0],r()}catch(e){r(e)}}))},686:(e,n,_)=>{_.a(e,(async(r,t)=>{try{_.d(n,{$7:()=>wn,$C:()=>Yn,$L:()=>n_,$d:()=>Xe,$o:()=>Qe,Ab:()=>Ne,CS:()=>Ce,DI:()=>Y,Dn:()=>cn,E7:()=>ce,EQ:()=>c_,Eo:()=>ue,G$:()=>xe,G6:()=>B,G8:()=>ln,H1:()=>f_,HT:()=>b_,If:()=>De,Il:()=>Cn,J5:()=>j,JH:()=>Tn,JT:()=>qe,JX:()=>Ae,Jn:()=>qn,Jx:()=>Ze,KC:()=>Ie,KM:()=>re,LD:()=>he,LQ:()=>Fn,Lh:()=>Oe,M$:()=>le,M1:()=>J,Me:()=>pe,Mk:()=>me,Mt:()=>Vn,Mv:()=>l_,Ni:()=>Kn,Nk:()=>R,Nl:()=>Re,O6:()=>dn,O_:()=>ge,Oh:()=>Me,Oq:()=>ke,Or:()=>y_,Oz:()=>$e,PP:()=>We,PX:()=>Pn,QT:()=>Z,QW:()=>En,Qc:()=>Ge,Qf:()=>we,Qy:()=>Se,RL:()=>a_,RU:()=>Dn,SF:()=>Fe,Sn:()=>e_,Sr:()=>w_,T7:()=>ye,TL:()=>I,TY:()=>i_,UO:()=>xn,VD:()=>t_,VI:()=>Be,Vx:()=>Zn,WM:()=>bn,Wl:()=>U,X$:()=>On,XM:()=>Ke,XP:()=>r_,XX:()=>fn,Y:()=>yn,Y7:()=>se,YL:()=>nn,ZR:()=>Qn,Z_:()=>un,Zs:()=>Pe,Zx:()=>s_,_n:()=>Sn,a2:()=>_e,ac:()=>Ue,am:()=>Mn,bB:()=>rn,bG:()=>M,bO:()=>Gn,bj:()=>g_,cE:()=>He,cq:()=>An,cr:()=>G,cs:()=>Ee,cw:()=>Rn,dC:()=>en,e:()=>H,eY:()=>N,em:()=>D,ex:()=>W,f5:()=>Q,fD:()=>ie,fP:()=>u_,fY:()=>p_,gM:()=>mn,gP:()=>Un,gW:()=>Nn,gj:()=>Ve,go:()=>K,h4:()=>q,hO:()=>Xn,hn:()=>L,iE:()=>O,iX:()=>te,ib:()=>m_,ig:()=>$n,ir:()=>oe,jY:()=>Te,jb:()=>ve,jp:()=>o_,kH:()=>h_,kJ:()=>pn,lB:()=>d_,lX:()=>be,m_:()=>ne,n4:()=>Wn,nh:()=>x_,o$:()=>z,oH:()=>v_,or:()=>In,p1:()=>ee,pB:()=>Le,q:()=>je,qP:()=>Ln,qh:()=>tn,qt:()=>V,r4:()=>an,rZ:()=>jn,rk:()=>ze,rw:()=>P_,sG:()=>vn,sK:()=>gn,sZ:()=>ae,sb:()=>Bn,tI:()=>_n,tX:()=>hn,tf:()=>sn,uR:()=>on,ug:()=>X,vK:()=>de,wJ:()=>__,w_:()=>Jn,wk:()=>Je,xA:()=>C,xH:()=>Ye,xJ:()=>kn,y3:()=>Hn,yB:()=>zn,yD:()=>S_,zi:()=>fe});var a=_(808);e=_.hmd(e);var o=r([a]);a=(o.then?(await o)():o)[0];const c=new Array(32).fill(void 0);function f(e){return c[e]}c.push(void 0,null,!0,!1);let b=c.length;function i(e){e<36||(c[e]=b,b=e)}function d(e){const n=f(e);return i(e),n}let u=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});u.decode();let g=new Uint8Array;function s(){return 0===g.byteLength&&(g=new Uint8Array(a.memory.buffer)),g}function w(e,n){return u.decode(s().subarray(e,e+n))}function l(e){b===c.length&&c.push(c.length+1);const n=b;return b=c[n],c[n]=e,n}let m=0,h=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const p="function"==typeof h.encodeInto?function(e,n){return h.encodeInto(e,n)}:function(e,n){const _=h.encode(e);return n.set(_),{read:e.length,written:_.length}};function y(e,n,_){if(void 0===_){const _=h.encode(e),r=n(_.length);return s().subarray(r,r+_.length).set(_),m=_.length,r}let r=e.length,t=n(r);const a=s();let o=0;for(;o<r;o++){const n=e.charCodeAt(o);if(n>127)break;a[t+o]=n}if(o!==r){0!==o&&(e=e.slice(o)),t=_(t,r,r=o+3*e.length);const n=s().subarray(t+o,t+r);o+=p(e,n).written}return m=o,t}function v(e){return null==e}let x=new Int32Array;function S(){return 0===x.byteLength&&(x=new Int32Array(a.memory.buffer)),x}let P=new Float64Array;function k(e){const n=typeof e;if("number"==n||"boolean"==n||null==e)return`${e}`;if("string"==n)return`"${e}"`;if("symbol"==n){const n=e.description;return null==n?"Symbol":`Symbol(${n})`}if("function"==n){const n=e.name;return"string"==typeof n&&n.length>0?`Function(${n})`:"Function"}if(Array.isArray(e)){const n=e.length;let _="[";n>0&&(_+=k(e[0]));for(let r=1;r<n;r++)_+=", "+k(e[r]);return _+="]",_}const _=/\[object ([^\]]+)\]/.exec(toString.call(e));let r;if(!(_.length>1))return toString.call(e);if(r=_[1],"Object"==r)try{return"Object("+JSON.stringify(e)+")"}catch(e){return"Object"}return e instanceof Error?`${e.name}: ${e.message}\n${e.stack}`:r}function A(e,n,_,r){const t={a:e,b:n,cnt:1,dtor:_},o=(...e)=>{t.cnt++;const n=t.a;t.a=0;try{return r(n,t.b,...e)}finally{0==--t.cnt?a.__wbindgen_export_2.get(t.dtor)(n,t.b):t.a=n}};return o.original=t,o}function E(e,n){a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h423807ed423ba7ce(e,n)}function T(e,n,_){a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h830cce78b1cf80cd(e,n,l(_))}function M(e){a.worker_set_global_scope(l(e))}function C(e){a.worker_on_message(l(e))}function I(e){a.set_worker(l(e))}function L(e){a.on_worker_message(l(e))}function O(e){a.set_scale(l(e))}function D(e){a.push_key_event(l(e))}function j(){try{const n=a.__wbindgen_add_to_stack_pointer(-16);a.main_wasm(n);var e=S()[n/4+0];if(S()[n/4+1])throw d(e)}finally{a.__wbindgen_add_to_stack_pointer(16)}}function F(e,n){try{return e.apply(this,n)}catch(e){a.__wbindgen_exn_store(l(e))}}function $(e,n){return s().subarray(e/1,e/1+n)}function B(e){const n=d(e).original;return 1==n.cnt--&&(n.a=0,!0)}function X(e){d(e)}function q(e,n){return l(w(e,n))}function V(e,n){const _=f(n),r="string"==typeof _?_:void 0;var t=v(r)?0:y(r,a.__wbindgen_malloc,a.__wbindgen_realloc),o=m;S()[e/4+1]=o,S()[e/4+0]=t}function J(e,n){const _=f(n),r="number"==typeof _?_:void 0;(0===P.byteLength&&(P=new Float64Array(a.memory.buffer)),P)[e/8+1]=v(r)?0:r,S()[e/4+0]=!v(r)}function W(){return F((function(e,n,_){f(e).randomFillSync($(n,_))}),arguments)}function H(){return F((function(e,n){f(e).getRandomValues(f(n))}),arguments)}function R(e){return l(f(e).crypto)}function U(e){const n=f(e);return"object"==typeof n&&null!==n}function Y(e){return l(f(e).process)}function Z(e){return l(f(e).versions)}function Q(e){return l(f(e).node)}function N(e){return"string"==typeof f(e)}function G(e){return l(f(e).msCrypto)}function K(){return F((function(){return l(e.require)}),arguments)}function z(e){return"function"==typeof f(e)}function ee(e,n){console.log(w(e,n))}function ne(e){return l(f(e))}function _e(){return l(new Error)}function re(e,n){const _=y(f(n).stack,a.__wbindgen_malloc,a.__wbindgen_realloc),r=m;S()[e/4+1]=r,S()[e/4+0]=_}function te(e,n){try{console.error(w(e,n))}finally{a.__wbindgen_free(e,n)}}function ae(e){const n=f(e);return"boolean"==typeof n?n?1:0:2}function oe(e){let n;try{n=f(e)instanceof WebGL2RenderingContext}catch{n=!1}return n}function ce(e,n){f(e).bindVertexArray(f(n))}function fe(e,n,_,r){f(e).bufferData(n>>>0,f(_),r>>>0)}function be(e){const n=f(e).createVertexArray();return v(n)?0:l(n)}function ie(){return F((function(e,n,_,r,t,a,o,c,b,i){f(e).texImage2D(n>>>0,_,r,t,a,o,c>>>0,b>>>0,f(i))}),arguments)}function de(e,n,_){f(e).attachShader(f(n),f(_))}function ue(e,n,_){f(e).bindBuffer(n>>>0,f(_))}function ge(e,n,_){f(e).bindFramebuffer(n>>>0,f(_))}function se(e,n,_){f(e).bindTexture(n>>>0,f(_))}function we(e,n,_){f(e).blendFunc(n>>>0,_>>>0)}function le(e,n){f(e).clear(n>>>0)}function me(e,n,_,r,t){f(e).clearColor(n,_,r,t)}function he(e,n){f(e).compileShader(f(n))}function pe(e){const n=f(e).createBuffer();return v(n)?0:l(n)}function ye(e){const n=f(e).createFramebuffer();return v(n)?0:l(n)}function ve(e){const n=f(e).createProgram();return v(n)?0:l(n)}function xe(e,n){const _=f(e).createShader(n>>>0);return v(_)?0:l(_)}function Se(e){const n=f(e).createTexture();return v(n)?0:l(n)}function Pe(e,n){f(e).disable(n>>>0)}function ke(e,n,_,r){f(e).drawArrays(n>>>0,_,r)}function Ae(e,n,_,r,t){f(e).drawElements(n>>>0,_,r>>>0,t)}function Ee(e,n){f(e).enable(n>>>0)}function Te(e,n){f(e).enableVertexAttribArray(n>>>0)}function Me(e,n,_,r,t,a){f(e).framebufferTexture2D(n>>>0,_>>>0,r>>>0,f(t),a)}function Ce(e){return f(e).getError()}function Ie(){return F((function(e,n,_){const r=f(e).getExtension(w(n,_));return v(r)?0:l(r)}),arguments)}function Le(e,n,_){const r=f(n).getProgramInfoLog(f(_));var t=v(r)?0:y(r,a.__wbindgen_malloc,a.__wbindgen_realloc),o=m;S()[e/4+1]=o,S()[e/4+0]=t}function Oe(e,n,_){return l(f(e).getProgramParameter(f(n),_>>>0))}function De(e,n,_){const r=f(n).getShaderInfoLog(f(_));var t=v(r)?0:y(r,a.__wbindgen_malloc,a.__wbindgen_realloc),o=m;S()[e/4+1]=o,S()[e/4+0]=t}function je(e,n,_){return l(f(e).getShaderParameter(f(n),_>>>0))}function Fe(e,n,_,r){const t=f(e).getUniformLocation(f(n),w(_,r));return v(t)?0:l(t)}function $e(e,n){f(e).linkProgram(f(n))}function Be(e,n,_,r){f(e).shaderSource(f(n),w(_,r))}function Xe(e,n,_,r){f(e).texParameteri(n>>>0,_>>>0,r)}function qe(e,n,_){f(e).uniform1i(f(n),_)}function Ve(e,n,_,r,t){f(e).uniform3f(f(n),_,r,t)}function Je(e,n){f(e).useProgram(f(n))}function We(e,n,_,r,t,a,o){f(e).vertexAttribPointer(n>>>0,_,r>>>0,0!==t,a,o)}function He(e){let n;try{n=f(e)instanceof Window}catch{n=!1}return n}function Re(e){const n=f(e).document;return v(n)?0:l(n)}function Ue(e){const n=f(e).performance;return v(n)?0:l(n)}function Ye(e,n){f(e).onkeydown=f(n)}function Ze(e,n){f(e).onkeyup=f(n)}function Qe(){return F((function(e,n){return f(e).requestAnimationFrame(f(n))}),arguments)}function Ne(e){return f(e).offsetX}function Ge(e){return f(e).offsetY}function Ke(e,n){f(e).bindVertexArrayOES(f(n))}function ze(e){const n=f(e).createVertexArrayOES();return v(n)?0:l(n)}function en(){return F((function(e,n){f(e).postMessage(f(n))}),arguments)}function nn(e){let n;try{n=f(e)instanceof HTMLCanvasElement}catch{n=!1}return n}function _n(e,n){f(e).width=n>>>0}function rn(e,n){f(e).height=n>>>0}function tn(){return F((function(e,n,_){const r=f(e).getContext(w(n,_));return v(r)?0:l(r)}),arguments)}function an(e){return f(e).now()}function on(){return F((function(e,n){f(e).postMessage(f(n))}),arguments)}function cn(e){return f(e).charCode}function fn(e){return f(e).keyCode}function bn(e,n){const _=y(f(n).code,a.__wbindgen_malloc,a.__wbindgen_realloc),r=m;S()[e/4+1]=r,S()[e/4+0]=_}function dn(e,n,_){return f(e).getModifierState(w(n,_))}function un(e,n,_){const r=f(e).getElementById(w(n,_));return v(r)?0:l(r)}function gn(e,n){f(e).onmousedown=f(n)}function sn(e,n){f(e).onmousemove=f(n)}function wn(e,n){f(e).onmouseup=f(n)}function ln(e,n,_,r){f(e).bufferData(n>>>0,f(_),r>>>0)}function mn(){return F((function(e,n,_,r,t,a,o,c,b,i){f(e).texImage2D(n>>>0,_,r,t,a,o,c>>>0,b>>>0,f(i))}),arguments)}function hn(e,n,_){f(e).attachShader(f(n),f(_))}function pn(e,n,_){f(e).bindBuffer(n>>>0,f(_))}function yn(e,n,_){f(e).bindFramebuffer(n>>>0,f(_))}function vn(e,n,_){f(e).bindTexture(n>>>0,f(_))}function xn(e,n,_){f(e).blendFunc(n>>>0,_>>>0)}function Sn(e,n){f(e).clear(n>>>0)}function Pn(e,n,_,r,t){f(e).clearColor(n,_,r,t)}function kn(e,n){f(e).compileShader(f(n))}function An(e){const n=f(e).createBuffer();return v(n)?0:l(n)}function En(e){const n=f(e).createFramebuffer();return v(n)?0:l(n)}function Tn(e){const n=f(e).createProgram();return v(n)?0:l(n)}function Mn(e,n){const _=f(e).createShader(n>>>0);return v(_)?0:l(_)}function Cn(e){const n=f(e).createTexture();return v(n)?0:l(n)}function In(e,n){f(e).disable(n>>>0)}function Ln(e,n,_,r){f(e).drawArrays(n>>>0,_,r)}function On(e,n,_,r,t){f(e).drawElements(n>>>0,_,r>>>0,t)}function Dn(e,n){f(e).enable(n>>>0)}function jn(e,n){f(e).enableVertexAttribArray(n>>>0)}function Fn(e,n,_,r,t,a){f(e).framebufferTexture2D(n>>>0,_>>>0,r>>>0,f(t),a)}function $n(e){return f(e).getError()}function Bn(e,n,_){const r=f(n).getProgramInfoLog(f(_));var t=v(r)?0:y(r,a.__wbindgen_malloc,a.__wbindgen_realloc),o=m;S()[e/4+1]=o,S()[e/4+0]=t}function Xn(e,n,_){return l(f(e).getProgramParameter(f(n),_>>>0))}function qn(e,n,_){const r=f(n).getShaderInfoLog(f(_));var t=v(r)?0:y(r,a.__wbindgen_malloc,a.__wbindgen_realloc),o=m;S()[e/4+1]=o,S()[e/4+0]=t}function Vn(e,n,_){return l(f(e).getShaderParameter(f(n),_>>>0))}function Jn(e,n,_,r){const t=f(e).getUniformLocation(f(n),w(_,r));return v(t)?0:l(t)}function Wn(e,n){f(e).linkProgram(f(n))}function Hn(e,n,_,r){f(e).shaderSource(f(n),w(_,r))}function Rn(e,n,_,r){f(e).texParameteri(n>>>0,_>>>0,r)}function Un(e,n,_){f(e).uniform1i(f(n),_)}function Yn(e,n,_,r,t){f(e).uniform3f(f(n),_,r,t)}function Zn(e,n){f(e).useProgram(f(n))}function Qn(e,n,_,r,t,a,o){f(e).vertexAttribPointer(n>>>0,_,r>>>0,0!==t,a,o)}function Nn(e,n){return l(new Function(w(e,n)))}function Gn(){return F((function(e,n){return l(Reflect.get(f(e),f(n)))}),arguments)}function Kn(){return F((function(e,n){return l(f(e).call(f(n)))}),arguments)}function zn(){return F((function(){return l(self.self)}),arguments)}function e_(){return F((function(){return l(window.window)}),arguments)}function n_(){return F((function(){return l(globalThis.globalThis)}),arguments)}function __(){return F((function(){return l(_.g.global)}),arguments)}function r_(e){return void 0===f(e)}function t_(){return F((function(e,n,_){return l(f(e).call(f(n),f(_)))}),arguments)}function a_(){return Date.now()}function o_(e){return l(f(e).buffer)}function c_(e,n,_){return l(new Int8Array(f(e),n>>>0,_>>>0))}function f_(e,n,_){return l(new Int16Array(f(e),n>>>0,_>>>0))}function b_(e,n,_){return l(new Int32Array(f(e),n>>>0,_>>>0))}function i_(e,n,_){return l(new Uint8Array(f(e),n>>>0,_>>>0))}function d_(e){return l(new Uint8Array(f(e)))}function u_(e,n,_){f(e).set(f(n),_>>>0)}function g_(e){return f(e).length}function s_(e,n,_){return l(new Uint16Array(f(e),n>>>0,_>>>0))}function w_(e,n,_){return l(new Uint32Array(f(e),n>>>0,_>>>0))}function l_(e,n,_){return l(new Float32Array(f(e),n>>>0,_>>>0))}function m_(e){return l(new Uint8Array(e>>>0))}function h_(e,n,_){return l(f(e).subarray(n>>>0,_>>>0))}function p_(e,n){const _=y(k(f(n)),a.__wbindgen_malloc,a.__wbindgen_realloc),r=m;S()[e/4+1]=r,S()[e/4+0]=_}function y_(e,n){throw new Error(w(e,n))}function v_(){return l(a.memory)}function x_(e,n,_){return l(A(e,n,73,E))}function S_(e,n,_){return l(A(e,n,858,T))}function P_(e,n,_){return l(A(e,n,858,T))}t()}catch(k_){t(k_)}}))},808:(e,n,_)=>{_.a(e,(async(r,t)=>{try{var a,o=r([a=_(686)]),[a]=o.then?(await o)():o;await _.v(n,e.id,"bb031235b2d14e552526",{"./index_bg.js":{__wbindgen_cb_drop:a.G6,__wbindgen_object_drop_ref:a.ug,__wbindgen_string_new:a.h4,__wbindgen_string_get:a.qt,__wbindgen_number_get:a.M1,__wbg_randomFillSync_6894564c2c334c42:a.ex,__wbg_getRandomValues_805f1c3d65988a5a:a.e,__wbg_crypto_e1d53a1d73fb10b8:a.Nk,__wbindgen_is_object:a.Wl,__wbg_process_038c26bf42b093f8:a.DI,__wbg_versions_ab37218d2f0b24a8:a.QT,__wbg_node_080f4b19d15bc1fe:a.f5,__wbindgen_is_string:a.eY,__wbg_msCrypto_6e7d3e1f92610cbb:a.cr,__wbg_require_78a3dcfbdba9cbce:a.go,__wbindgen_is_function:a.o$,__wbg_log_b09521c515df0f23:a.p1,__wbindgen_object_clone_ref:a.m_,__wbg_new_abda76e883ba8a5f:a.a2,__wbg_stack_658279fe44541cf6:a.KM,__wbg_error_f851667af71bcfc6:a.iX,__wbindgen_boolean_get:a.sZ,__wbg_instanceof_WebGl2RenderingContext_fcfa91cd777063f3:a.ir,__wbg_bindVertexArray_9d12800e272184b0:a.E7,__wbg_bufferData_8d206d7adf6751c0:a.zi,__wbg_createVertexArray_8467a75e68fec199:a.lX,__wbg_texImage2D_1bc6fe2370a72e1c:a.fD,__wbg_attachShader_90ad543fb1bccb18:a.vK,__wbg_bindBuffer_66e359418f5c82d7:a.Eo,__wbg_bindFramebuffer_5c01742edd5d843a:a.O_,__wbg_bindTexture_ae9620ea4a6ffb97:a.Y7,__wbg_blendFunc_99b48b64bde98c6f:a.Qf,__wbg_clear_05614d3b84e96aae:a.M$,__wbg_clearColor_bc89a6580c0498c3:a.Mk,__wbg_compileShader_822f38928f6f2a08:a.LD,__wbg_createBuffer_a6cffb7f7d5b92a3:a.Me,__wbg_createFramebuffer_d5f3985ce3652661:a.T7,__wbg_createProgram_dc6b23d3caa1d86e:a.jb,__wbg_createShader_46a66dce5a9e22d0:a.G$,__wbg_createTexture_269f67d411bdc4dc:a.Qy,__wbg_disable_1659dc1efb5fb934:a.Zs,__wbg_drawArrays_d587302f7a868d91:a.Oq,__wbg_drawElements_241caa588795bcb1:a.JX,__wbg_enable_4791414dce6f602a:a.cs,__wbg_enableVertexAttribArray_a1ffc091f3999354:a.jY,__wbg_framebufferTexture2D_4b810902dffa1ef3:a.Oh,__wbg_getError_8de2be43ffb2c4e0:a.CS,__wbg_getExtension_e7912bce04869d40:a.KC,__wbg_getProgramInfoLog_1e37a3d1d090ec1c:a.pB,__wbg_getProgramParameter_acf4ae158143e2b2:a.Lh,__wbg_getShaderInfoLog_451545b963646762:a.If,__wbg_getShaderParameter_6cd8c36fded266ea:a.q,__wbg_getUniformLocation_0da0c93f626244a2:a.SF,__wbg_linkProgram_c33885d9ea798810:a.Oz,__wbg_shaderSource_5111981e7afb61fb:a.VI,__wbg_texParameteri_21fd6b6b394882c9:a.$d,__wbg_uniform1i_49986febd844f2c4:a.JT,__wbg_uniform3f_d756c07788fa91da:a.gj,__wbg_useProgram_35a58ac1e0d9577b:a.wk,__wbg_vertexAttribPointer_3b06d737566f0745:a.PP,__wbg_instanceof_Window_acc97ff9f5d2c7b4:a.cE,__wbg_document_3ead31dbcad65886:a.Nl,__wbg_performance_de9825f9a8678574:a.ac,__wbg_setonkeydown_ddc0009c6d7693cd:a.xH,__wbg_setonkeyup_4e1eff214c25854c:a.Jx,__wbg_requestAnimationFrame_4181656476a7d86c:a.$o,__wbg_offsetX_8891849b36542d53:a.Ab,__wbg_offsetY_1f52082687af467b:a.Qc,__wbg_bindVertexArrayOES_84540c072ea96b75:a.XM,__wbg_createVertexArrayOES_00a5c523e5b17eff:a.rk,__wbg_postMessage_ef1cef7e229c46d7:a.dC,__wbg_instanceof_HtmlCanvasElement_97761617af6ea089:a.YL,__wbg_setwidth_afb418d3fbf71ba7:a.tI,__wbg_setheight_3eb8729b59493242:a.bB,__wbg_getContext_4d5e97892c1b206a:a.qh,__wbg_now_8172cd917e5eda6b:a.r4,__wbg_postMessage_8a8f33d997e17e7b:a.uR,__wbg_charCode_b0f31612a52c2bff:a.Dn,__wbg_keyCode_72faed4278f77f2c:a.XX,__wbg_code_06787cd3c7a60600:a.WM,__wbg_getModifierState_135305ae40997dc7:a.O6,__wbg_getElementById_3a708b83e4f034d7:a.Z_,__wbg_setonmousedown_8778ff85c1c9cd52:a.sK,__wbg_setonmousemove_ef55e5efd369f524:a.tf,__wbg_setonmouseup_39fd2509e8c154de:a.$7,__wbg_bufferData_a33528a74dd300f4:a.G8,__wbg_texImage2D_5b25282e44d0e3fe:a.gM,__wbg_attachShader_f4d51147351a1906:a.tX,__wbg_bindBuffer_8b5135aa633680f5:a.kJ,__wbg_bindFramebuffer_080d0b0cf22e1645:a.Y,__wbg_bindTexture_6f1dec563e82e818:a.sG,__wbg_blendFunc_49ea28240d4c1084:a.UO,__wbg_clear_576f67967748e95f:a._n,__wbg_clearColor_7489a3fbe484f2f1:a.PX,__wbg_compileShader_22b038faa1f49857:a.xJ,__wbg_createBuffer_6e747d928c9ba46d:a.cq,__wbg_createFramebuffer_9b5b0507480146cd:a.QW,__wbg_createProgram_1c5f8dffd1066e71:a.JH,__wbg_createShader_4017d9fbc36659af:a.am,__wbg_createTexture_4ce49e8a8c655124:a.Il,__wbg_disable_6835d16c2cd3fa26:a.or,__wbg_drawArrays_c0dcb4151e0bf007:a.qP,__wbg_drawElements_e09dbef58c8f099a:a.X$,__wbg_enable_fc393941ac400f72:a.RU,__wbg_enableVertexAttribArray_3d21f4936ad4a378:a.rZ,__wbg_framebufferTexture2D_499d1c21458d0113:a.LQ,__wbg_getError_9ace44157772dd10:a.ig,__wbg_getProgramInfoLog_e47d5073d57fb18d:a.sb,__wbg_getProgramParameter_eaf768a9b399b7cf:a.hO,__wbg_getShaderInfoLog_ec7e5b959e47645b:a.Jn,__wbg_getShaderParameter_42a35b974329561c:a.Mt,__wbg_getUniformLocation_8e9cc276a231ddcd:a.w_,__wbg_linkProgram_25cda5f9318ea316:a.n4,__wbg_shaderSource_a0001b8eab5d44f4:a.y3,__wbg_texParameteri_1b210b807f1ea723:a.cw,__wbg_uniform1i_50124a48de1da66b:a.gP,__wbg_uniform3f_35a7a76696c08aa4:a.$C,__wbg_useProgram_156511a425feb519:a.Vx,__wbg_vertexAttribPointer_63d2aef49627302b:a.ZR,__wbg_newnoargs_b5b063fc6c2f0376:a.gW,__wbg_get_765201544a2b6869:a.bO,__wbg_call_97ae9d8645dc388b:a.Ni,__wbg_self_6d479506f72c6a71:a.yB,__wbg_window_f2557cc78490aceb:a.Sn,__wbg_globalThis_7f206bda628d5286:a.$L,__wbg_global_ba75c50d1cf384f4:a.wJ,__wbindgen_is_undefined:a.XP,__wbg_call_168da88779e35f61:a.VD,__wbg_now_58886682b7e790d7:a.RL,__wbg_buffer_3f3d764d4747d564:a.jp,__wbg_newwithbyteoffsetandlength_d9aa266703cb98be:a.TY,__wbg_new_8c3f0052272a457a:a.lB,__wbg_set_83db9690f9353e79:a.fP,__wbg_length_9e1ae1900cb0fbd5:a.bj,__wbg_newwithlength_f5933855e4f48a19:a.ib,__wbg_subarray_58ad4efbb5bcb886:a.kH,__wbindgen_debug_string:a.fY,__wbindgen_throw:a.Or,__wbindgen_memory:a.oH,__wbindgen_closure_wrapper359:a.nh,__wbindgen_closure_wrapper2330:a.yD,__wbindgen_closure_wrapper2332:a.rw}}),t()}catch(e){t(e)}}),1)}},a={};function o(e){var n=a[e];if(void 0!==n)return n.exports;var _=a[e]={id:e,loaded:!1,exports:{}};return t[e](_,_.exports,o),_.loaded=!0,_.exports}o.m=t,e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",_="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&!e.d&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},o.a=(t,a,o)=>{var c;o&&((c=[]).d=1);var f,b,i,d=new Set,u=t.exports,g=new Promise(((e,n)=>{i=n,b=e}));g[n]=u,g[e]=e=>(c&&e(c),d.forEach(e),g.catch((e=>{}))),t.exports=g,a((t=>{var a;f=(t=>t.map((t=>{if(null!==t&&"object"==typeof t){if(t[e])return t;if(t.then){var a=[];a.d=0,t.then((e=>{o[n]=e,r(a)}),(e=>{o[_]=e,r(a)}));var o={};return o[e]=e=>e(a),o}}var c={};return c[e]=e=>{},c[n]=t,c})))(t);var o=()=>f.map((e=>{if(e[_])throw e[_];return e[n]})),b=new Promise((n=>{(a=()=>n(o)).r=0;var _=e=>e!==c&&!d.has(e)&&(d.add(e),e&&!e.d&&(a.r++,e.push(a)));f.map((n=>n[e](_)))}));return a.r?b:o()}),(e=>(e?i(g[_]=e):b(u),r(c)))),c&&(c.d=0)},o.n=e=>{var n=e&&e.__esModule?()=>e.default:()=>e;return o.d(n,{a:n}),n},o.d=(e,n)=>{for(var _ in n)o.o(n,_)&&!o.o(e,_)&&Object.defineProperty(e,_,{enumerable:!0,get:n[_]})},o.u=e=>e+".app.js",o.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),o.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),o.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),o.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},o.v=(e,n,_,r)=>{var t=fetch(o.p+""+_+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,r).then((n=>Object.assign(e,n.instance.exports))):t.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)))},(()=>{var e;o.g.importScripts&&(e=o.g.location+"");var n=o.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var _=n.getElementsByTagName("script");_.length&&(e=_[_.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),o.p=e})(),o.b=document.baseURI||self.location.href,o.nc=void 0,(()=>{var e=o(379),n=o.n(e),_=o(795),r=o.n(_),t=o(569),a=o.n(t),c=o(565),f=o.n(c),b=o(216),i=o.n(b),d=o(589),u=o.n(d),g=o(982),s={};function w(e){const n=1440,_=(document.documentElement.clientWidth??window.innerWidth??0)-20,r=(document.documentElement.clientHeight??window.innerHeight??0)-20;let t=800,a=n;n<=_&&800<=r||(_/r>=1.8?(t=Math.min(r,800),a=1.8*t):(a=Math.min(_,n),t=a/1.8));let o=document.querySelector(":root");o.style.setProperty("--term-width",a+"px"),o.style.setProperty("--term-height",t+"px"),e.set_scale(a/n),console.log(a/n)}s.styleTagTransform=u(),s.setAttributes=f(),s.insert=a().bind(null,"head"),s.domAPI=r(),s.insertStyleElement=i(),n()(g.Z,s),g.Z&&g.Z.locals&&g.Z.locals,async function(){try{let n=o(391),_=await n;_.main_wasm(),window.addEventListener("resize",(()=>w(_))),w(_),e=_,document.addEventListener("keypress",(n=>{var _=n.key;e.push_key_event(_),n.preventDefault()}),!1),document.addEventListener("keydown",(n=>{var _=n.key;"Backspace"==_&&(e.push_key_event(_),n.preventDefault())}),!1);let r=await async function(){const e=new Worker(new URL(o.p+o.u(152),o.b));return await new Promise(((n,_)=>{e.onmessage=({data:r})=>{e.onmessage=null,"initialized"===r?n():_()}})),e.onmessage=({data:e})=>{console.log(e)},e}();_.set_worker(r),r.onmessage=({data:e})=>{console.log("main received: ",e),_.on_worker_message(e)}}catch(e){!function(e){document.getElementById("canvas").style.display="none",document.getElementById("canvas-placeholder").getElementsByClassName("title")[0].innerHTML="Error",document.getElementsByClassName("error")[0].innerHTML="Unhandled error in the WASM backend!<br/><em>&gt; "+e+"</em><br/><br/>Check dev console for more info.<br/><br/>"}(e)}var e}()})()})();