let X=`function`,Y=`number`,R=1,a0=`Object`,_=`string`,a3=4,Z=`boolean`,V=0,Q=null,a2=16,T=`utf-8`,a6=15,S=`undefined`,O=Array,$=Array.isArray,U=Error,a1=FinalizationRegistry,a5=Promise,a4=Reflect,W=Uint8Array,P=undefined;var I=(async(a,b)=>{if(typeof Response===X&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===X){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=(a=>{const b=typeof a;if(b==Y||b==Z||a==Q){return `${a}`};if(b==_){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==Q){return `Symbol`}else{return `Symbol(${b})`}};if(b==X){const b=a.name;if(typeof b==_&&b.length>V){return `Function(${b})`}else{return `Function`}};if($(a)){const b=a.length;let c=`[`;if(b>V){c+=u(a[V])};for(let d=R;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>R){d=c[R]}else{return toString.call(a)};if(d==a0){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return a0}};if(a instanceof U){return `${a.name}: ${a.message}\n${a.stack}`};return d});var e=(a=>{if(a<132)return;b[a]=d;d=a});function A(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(g(b))}}var p=(a=>a===P||a===Q);var t=(()=>{if(s===Q||s.byteLength===V){s=new Float64Array(a.memory.buffer)};return s});var o=((a,b,c)=>{if(c===P){const c=m.encode(a);const d=b(c.length,R)>>>V;j().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,R)>>>V;const f=j();let g=V;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==V){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,R)>>>V;const b=j().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written};l=g;return e});var k=((a,b)=>{a=a>>>V;return h.decode(j().subarray(a,a+ b))});var J=(()=>{const b={};b.wbg={};b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return p(b)?V:g(b)});b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return g(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return g(c)});b.wbg.__wbg_set_37a50e901587b477=function(){return A(((a,b,d)=>{const e=a4.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbg_addEventListener_1b158e9e95e0ab00=function(){return A(((a,b,d,e,f)=>{var g=z(b,d);c(a).addEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_documentElement_a030fb9a26624716=(a=>{const b=c(a).documentElement;return p(b)?V:g(b)});b.wbg.__wbg_querySelector_d86f889797c65e88=function(){return A(((a,b,d)=>{var e=z(b,d);const f=c(a).querySelector(e);return p(f)?V:g(f)}),arguments)};b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new U();return g(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{var d=z(b,c);if(b!==V){a.__wbindgen_free(b,c,R)};console.error(d)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===_;return b});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===X;return b});b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return A(((a,b)=>{const d=c(a).call(c(b));return g(d)}),arguments)};b.wbg.__wbg_get_5027b32da70f39b1=function(){return A(((a,b)=>{const d=a4.get(c(a),c(b));return g(d)}),arguments)};b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===_?e:P;var g=p(f)?V:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/a3+ R]=h;r()[b/a3+ V]=g});b.wbg.__wbg_self_086b5302bcafb962=function(){return A((()=>{const a=self.self;return g(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return A((()=>{const a=window.window;return g(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return A((()=>{const a=globalThis.globalThis;return g(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return A((()=>{const a=global.global;return g(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===P;return b});b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{var c=z(a,b);const d=new Function(c);return g(d)});b.wbg.__wbg_decodeURI_4bf318e70843e94f=function(){return A(((a,b)=>{var c=z(a,b);const d=decodeURI(c);return g(d)}),arguments)};b.wbg.__wbg_call_f6a2bc58c19c53c6=function(){return A(((a,b,d)=>{const e=c(a).call(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=Object.is(c(a),c(b));return d});b.wbg.__wbg_exec_70dc2e84f7a29584=((a,b,d)=>{var e=z(b,d);const f=c(a).exec(e);return p(f)?V:g(f)});b.wbg.__wbg_new_c62202c5c4bd9009=((a,b,c,d)=>{var e=z(a,b);var f=z(c,d);const h=new RegExp(e,f);return g(h)});b.wbg.__wbg_remove_0df84ff63b459921=function(){return A(((a,b,d)=>{var e=z(b,d);c(a).remove(e)}),arguments)};b.wbg.__wbg_add_44212bfb95df48ba=function(){return A(((a,b,d)=>{var e=z(b,d);c(a).add(e)}),arguments)};b.wbg.__wbg_error_1f4e3e298a7c97f6=(a=>{console.error(c(a))});b.wbg.__wbg_setdata_4d5b377238fff97c=((a,b,d)=>{var e=z(b,d);c(a).data=e});b.wbg.__wbg_previousSibling_4cd9e84aeb4df529=(a=>{const b=c(a).previousSibling;return p(b)?V:g(b)});b.wbg.__wbg_remove_ed2f62f1a8be044b=(a=>{c(a).remove()});b.wbg.__wbg_before_bed7b7b6e53dd469=function(){return A(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_childNodes_75d3da5f3a7bb985=(a=>{const b=c(a).childNodes;return g(b)});b.wbg.__wbg_length_d5ed87010607a669=(a=>{const b=c(a).length;return b});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return p(b)?V:g(b)});b.wbg.__wbg_createComment_ce9f467394242d45=((a,b,d)=>{var e=z(b,d);const f=c(a).createComment(e);return g(f)});b.wbg.__wbg_target_791826e938c3e308=(a=>{const b=c(a).target;return p(b)?V:g(b)});b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return g(b)});b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>V];return g(d)});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbg_cancelBubble_191799b8e0ab3254=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_parentNode_f3957fdd408a62f7=(a=>{const b=c(a).parentNode;return p(b)?V:g(b)});b.wbg.__wbg_instanceof_ShadowRoot_cb6366cb0956ce29=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_99e27ed8897850f2=(a=>{const b=c(a).host;return g(b)});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===Q;return b});b.wbg.__wbg_createDocumentFragment_229f723f44e69ab9=(a=>{const b=c(a).createDocumentFragment();return g(b)});b.wbg.__wbg_location_0f233324e8e8c699=(a=>{const b=c(a).location;return g(b)});b.wbg.__wbg_requestAnimationFrame_1820a8e6b645ec5a=function(){return A(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_removeEventListener_07715e6f464823fc=function(){return A(((a,b,d,e)=>{var f=z(b,d);c(a).removeEventListener(f,c(e))}),arguments)};b.wbg.__wbg_warn_0e0204547af47087=(a=>{console.warn(c(a))});b.wbg.__wbg_classList_b75072943b838f29=(a=>{const b=c(a).classList;return g(b)});b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{var e=z(b,d);const f=c(a).createTextNode(e);return g(f)});b.wbg.__wbg_settextContent_1fec240f77aa3dc4=((a,b,d)=>{var e=z(b,d);c(a).textContent=e});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==R){b.a=V;return !0};const c=!1;return c});b.wbg.__wbg_pathname_d0d5b2fd2c7d8243=((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbg_search_b5c7b044aaf64616=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbg_searchParams_40c1f8d0a85de0aa=(a=>{const b=c(a).searchParams;return g(b)});b.wbg.__wbg_iterator_364187e1ee96b750=(()=>{const a=Symbol.iterator;return g(a)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==Q;return d});b.wbg.__wbg_next_1938cf110c9491d4=(a=>{const b=c(a).next;return g(b)});b.wbg.__wbg_next_267398d0e0761bf9=function(){return A((a=>{const b=c(a).next();return g(b)}),arguments)};b.wbg.__wbg_done_506b44765ba84b9c=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_31485d8770eb06ab=(a=>{const b=c(a).value;return g(b)});b.wbg.__wbg_isArray_fbd24d447869b527=(a=>{const b=$(c(a));return b});b.wbg.__wbg_hash_286eced2921b7b34=((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbg_scrollIntoView_31df5d1d2bc727ab=(a=>{c(a).scrollIntoView()});b.wbg.__wbg_scrollTo_fa65cff02c3e7c5a=((a,b,d)=>{c(a).scrollTo(b,d)});b.wbg.__wbindgen_jsval_eq=((a,b)=>{const d=c(a)===c(b);return d});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===Y?d:P;t()[a/8+ R]=p(e)?V:e;r()[a/a3+ V]=!p(e)});b.wbg.__wbg_sethref_1d1a687bb352406f=function(){return A(((a,b,d)=>{var e=z(b,d);c(a).href=e}),arguments)};b.wbg.__wbg_defaultPrevented_37035afe196e2aa0=(a=>{const b=c(a).defaultPrevented;return b});b.wbg.__wbg_button_8a97c55db17c7314=(a=>{const b=c(a).button;return b});b.wbg.__wbg_metaKey_4e3f6e986f2802b1=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_altKey_a076f8612103d7e8=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_ctrlKey_0d75e0e9028bd999=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_12353f0e19b21d6a=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_length_d99b680fd68bf71b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_href_596cc35e5517d06b=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbg_target_a012ddff98c775bb=((b,d)=>{const e=c(d).target;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbg_getAttribute_14ccdc738c4f7d95=((b,d,e,f)=>{var g=z(e,f);const h=c(d).getAttribute(g);var i=p(h)?V:o(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=l;r()[b/a3+ R]=j;r()[b/a3+ V]=i});b.wbg.__wbg_preventDefault_d2c7416966cb0632=(a=>{c(a).preventDefault()});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===Z?(b?R:V):2;return d});b.wbg.__wbg_instanceof_HtmlAnchorElement_afbec6867d883d45=(a=>{let b;try{b=c(a) instanceof HTMLAnchorElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_pushState_90b0a1cf59505502=function(){return A(((a,b,d,e,f,g)=>{var h=z(d,e);var i=z(f,g);c(a).pushState(c(b),h,i)}),arguments)};b.wbg.__wbg_replaceState_594fcc05da10fe45=function(){return A(((a,b,d,e,f,g)=>{var h=z(d,e);var i=z(f,g);c(a).replaceState(c(b),h,i)}),arguments)};b.wbg.__wbg_state_cabf8868613a7bdb=function(){return A((a=>{const b=c(a).state;return g(b)}),arguments)};b.wbg.__wbg_pathname_2cd8b46817926b06=function(){return A(((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f}),arguments)};b.wbg.__wbg_search_eb68df82d26f8761=function(){return A(((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new U(k(a,b))});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return g(d)});b.wbg.__wbg_queueMicrotask_4d890031a6a5a50c=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_adae4bc085237231=(a=>{const b=c(a).queueMicrotask;return g(b)});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=a5.resolve(c(a));return g(b)});b.wbg.__wbg_byobRequest_004146c1db53bc14=(a=>{const b=c(a).byobRequest;return p(b)?V:g(b)});b.wbg.__wbg_view_d7afa0120e493b2d=(a=>{const b=c(a).view;return p(b)?V:g(b)});b.wbg.__wbg_byteLength_a8d894d93425b2e0=(a=>{const b=c(a).byteLength;return b});b.wbg.__wbg_close_54a5b70c42a72ee3=function(){return A((a=>{c(a).close()}),arguments)};b.wbg.__wbg_new_3a66822ed076951c=((a,b)=>{var c=z(a,b);const d=new U(c);return g(d)});b.wbg.__wbg_buffer_3da2aecfd9814cd8=(a=>{const b=c(a).buffer;return g(b)});b.wbg.__wbg_byteOffset_89d0a5265d5bde53=(a=>{const b=c(a).byteOffset;return b});b.wbg.__wbg_newwithbyteoffsetandlength_d695c7957788f922=((a,b,d)=>{const e=new W(c(a),b>>>V,d>>>V);return g(e)});b.wbg.__wbg_length_f0764416ba5bb237=(a=>{const b=c(a).length;return b});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return g(b)});b.wbg.__wbg_buffer_5d1b598a01b41a42=(a=>{const b=c(a).buffer;return g(b)});b.wbg.__wbg_set_74906aa30864df5a=((a,b,d)=>{c(a).set(c(b),d>>>V)});b.wbg.__wbg_close_21d8fce01634cc74=function(){return A((a=>{c(a).close()}),arguments)};b.wbg.__wbg_enqueue_61ebfae3475d5d91=function(){return A(((a,b)=>{c(a).enqueue(c(b))}),arguments)};b.wbg.__wbg_new_1d93771b84541aa5=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=V;try{return B(d,c.b,a,b)}finally{c.a=d}};const e=new a5(d);return g(e)}finally{c.a=c.b=V}});b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_head_267359b89c3f0368=(a=>{const b=c(a).head;return p(b)?V:g(b)});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return A(((a,b,d)=>{var e=z(b,d);const f=c(a).createElement(e);return g(f)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return A(((a,b,d,e,f)=>{var h=z(b,d);var i=z(e,f);const j=c(a).createElementNS(h,i);return g(j)}),arguments)};b.wbg.__wbg_getElementById_00904c7c4a32c23b=((a,b,d)=>{var e=z(b,d);const f=c(a).getElementById(e);return p(f)?V:g(f)});b.wbg.__wbg_append_517583bac5b5bb16=function(){return A(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{var e=z(b,d);c(a).innerHTML=e});b.wbg.__wbg_hasAttribute_40947267e195b26c=((a,b,d)=>{var e=z(b,d);const f=c(a).hasAttribute(e);return f});b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return A(((a,b,d)=>{var e=z(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return A(((a,b,d,e,f)=>{var g=z(b,d);var h=z(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_addEventListener_2f891d22985fd3c8=function(){return A(((a,b,d,e)=>{var f=z(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_origin_a66ff95a994d7e40=function(){return A(((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f}),arguments)};b.wbg.__wbg_hash_9bd16c0f666cdf27=function(){return A(((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f}),arguments)};b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return p(b)?V:g(b)});b.wbg.__wbg_appendChild_bd383ec5356c0bdb=function(){return A(((a,b)=>{const d=c(a).appendChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_cloneNode_80501c66ab115588=function(){return A((a=>{const b=c(a).cloneNode();return g(b)}),arguments)};b.wbg.__wbg_removeChild_14b08321b677677a=function(){return A(((a,b)=>{const d=c(a).removeChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_respond_3233ecfa19b9b617=function(){return A(((a,b)=>{c(a).respond(b>>>V)}),arguments)};b.wbg.__wbg_href_aa2244ca34a67d87=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbg_origin_028cdb6d9987f6c3=((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a3+ R]=g;r()[b/a3+ V]=f});b.wbg.__wbg_newwithbase_1151109a3f062f92=function(){return A(((a,b,c,d)=>{var e=z(a,b);var f=z(c,d);const h=new URL(e,f);return g(h)}),arguments)};b.wbg.__wbg_history_370f36be0803466b=function(){return A((a=>{const b=c(a).history;return g(b)}),arguments)};b.wbg.__wbindgen_closure_wrapper224=((a,b,c)=>{const d=w(a,b,a6,x);return g(d)});b.wbg.__wbindgen_closure_wrapper326=((a,b,c)=>{const d=w(a,b,39,x);return g(d)});b.wbg.__wbindgen_closure_wrapper588=((a,b,c)=>{const d=w(a,b,42,y);return g(d)});b.wbg.__wbindgen_closure_wrapper1392=((a,b,c)=>{const d=w(a,b,a6,y);return g(d)});b.wbg.__wbindgen_closure_wrapper1853=((a,b,c)=>{const d=w(a,b,a6,x);return g(d)});return b});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:R,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=V;try{return e(c,f.b,...b)}finally{if(--f.cnt===V){a.__wbindgen_export_2.get(f.dtor)(c,f.b);v.unregister(f)}else{f.a=c}}};g.original=f;v.register(g,f,f);return g});var f=(a=>{const b=c(a);e(a);return b});var c=(a=>b[a]);var x=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h37143d03aa9b9ae6(b,c,g(d))});var y=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0c95443601ae99ee(b,c)});var M=(b=>{if(a!==P)return a;const c=J();K(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return L(d,b)});var j=(()=>{if(i===Q||i.byteLength===V){i=new W(a.memory.buffer)};return i});var L=((b,c)=>{a=b.exports;N.__wbindgen_wasm_module=c;s=Q;q=Q;i=Q;a.__wbindgen_start();return a});var N=(async(b)=>{if(a!==P)return a;if(typeof b===S){b=new URL(`armaanrashid_bg.wasm`,import.meta.url)};const c=J();if(typeof b===_||typeof Request===X&&b instanceof Request||typeof URL===X&&b instanceof URL){b=fetch(b)};K(c);const {instance:d,module:e}=await I(await b,c);return L(d,e)});var K=((a,b)=>{});var B=((b,c,d,e)=>{a.wasm_bindgen__convert__closures__invoke2_mut__h9391d78fd13b7a55(b,c,g(d),g(e))});var z=((a,b)=>{if(a===V){return c(b)}else{return k(a,b)}});var r=(()=>{if(q===Q||q.byteLength===V){q=new Int32Array(a.memory.buffer)};return q});var g=(a=>{if(d===b.length)b.push(b.length+ R);const c=d;d=b[c];b[c]=a;return c});let a;const b=new O(128).fill(P);b.push(P,Q,!0,!1);let d=b.length;const h=typeof TextDecoder!==S?new TextDecoder(T,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw U(`TextDecoder not available`)}};if(typeof TextDecoder!==S){h.decode()};let i=Q;let l=V;const m=typeof TextEncoder!==S?new TextEncoder(T):{encode:()=>{throw U(`TextEncoder not available`)}};const n=typeof m.encodeInto===X?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=Q;let s=Q;const v=new a1(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});const C=new a1(b=>a.__wbg_intounderlyingbytesource_free(b>>>V));class D{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=V;C.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b)}type(){try{const e=a.__wbindgen_add_to_stack_pointer(-a2);a.intounderlyingbytesource_type(e,this.__wbg_ptr);var b=r()[e/a3+ V];var c=r()[e/a3+ R];var d=z(b,c);if(b!==V){a.__wbindgen_free(b,c,R)};return d}finally{a.__wbindgen_add_to_stack_pointer(a2)}}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>V}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,g(b))}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,g(b));return f(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}const E=new a1(b=>a.__wbg_intounderlyingsink_free(b>>>V));class F{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=V;E.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,g(b));return f(c)}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return f(c)}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,g(b));return f(d)}}const G=new a1(b=>a.__wbg_intounderlyingsource_free(b>>>V));class H{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=V;G.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,g(b));return f(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default N;export{D as IntoUnderlyingByteSource,F as IntoUnderlyingSink,H as IntoUnderlyingSource,M as initSync}