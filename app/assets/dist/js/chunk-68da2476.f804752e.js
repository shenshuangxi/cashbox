(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-68da2476"],{"0147":function(t,e,i){i("a29f"),i("e60f")},"21d5":function(t,e,i){"use strict";i.r(e);var a=function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("div",{staticClass:"authorize_state_page"},[i("div",{staticClass:"auth_aims_group"},[i("span",{staticClass:"auth_aims_title_key"},[t._v(t._s(t.$t("original.operationType"))+" ")]),i("span",{staticClass:"auth_aims_title_value"},[t._v(t._s(t.authTypeText))])]),i("div",{staticClass:"white_divide_section"}),i("div",{staticClass:"auth_state_group"},[i("span",{staticClass:"auth_state_title"},[t._v(t._s(t.$t("original.statesList")))])]),t._l(this.originAddressList,(function(e,a){return i("div",{staticClass:"address_item"},[i("div",{staticClass:"address_item_content",class:{address_item_content_active:!0===e.signStatus}},[i("div",{staticClass:"address_title_group"},[i("span",{staticClass:"item_address_content"},[t._v(t._s(t.$t("original.authAddress"))+t._s(a+1)+":")]),i("van-button",{staticClass:"apply_title_button",class:{apply_title_button_active:!0===e.signStatus},attrs:{size:"small",disabled:!1},on:{click:function(e){return t.applyForSign(a)}}},[t._v("\n                    申请授权\n                ")])],1),i("div",{staticClass:"admin_info_group"},[i("van-collapse",{model:{value:t.activeNames,callback:function(e){t.activeNames=e},expression:"activeNames"}},[i("van-collapse-item",{attrs:{name:a}},[i("div",{staticClass:"admin_name_key",attrs:{slot:"title"},slot:"title"},[t._v("用户名")]),i("div",{staticClass:"admin_name_value",attrs:{slot:"value"},slot:"value"},[t._v("\n                            "+t._s(e.name)+"\n                        ")]),i("div",{staticClass:"admin_address_group"},[i("span",{staticClass:"admin_address_key",style:{textAlign:"left",display:"inline-block",paddingLeft:"1%"}},[t._v("地址")]),i("span",{staticClass:"admin_address_value"},[t._v(t._s(e.address))])])])],1)],1),i("div",{staticClass:"state_group"},[i("div",{staticClass:"apply_state_group"},[i("span",{staticClass:"apply_state_key"},[t._v(t._s(t.$t("original.status")))]),i("span",{staticClass:"apply_state_value",class:{apply_state_value_active:!0===e.signStatus}},[t._v("\n                        "+t._s(!1===e.signStatus?"未授权":"已授权")+"\n                    ")])])]),i("div",{style:{height:"10px",backgroundColor:"#F8F8F8"}})])])})),i("div",{staticClass:"confirm_button_group"},[i("van-button",{staticClass:"confirm_button",attrs:{plain:"",type:"info"},on:{click:this.upTxToChain}},[t._v("输入密码，签名上链")])],1),i("van-overlay",{attrs:{show:t.isShowLoading}},[i("van-loading",{staticClass:"loading_component",attrs:{type:"spinner",vertical:""}},[t._v("信息上链进行中...")])],1)],2)},n=[],s=(i("96cf"),i("3b8d")),o=(i("7f7f"),i("ac6a"),i("6b54"),i("d225")),r=i("b0b4"),l=i("308d"),d=i("6bb5"),c=i("4e2b"),u=i("9ab4"),h=i("60a3"),f=i("ea47"),p=i("b650"),v=i("9ed2"),g=i("d282"),b=i("9884"),m=i("b1d2"),_=Object(g["a"])("collapse"),x=_[0],y=_[1],T=x({mixins:[Object(b["b"])("vanCollapse")],props:{accordion:Boolean,value:[String,Number,Array],border:{type:Boolean,default:!0}},methods:{switch:function(t,e){this.accordion||(t=e?this.value.concat(t):this.value.filter((function(e){return e!==t}))),this.$emit("change",t),this.$emit("input",t)}},render:function(){var t,e=arguments[0];return e("div",{class:[y(),(t={},t[m["d"]]=this.border,t)]},[this.slots()])}}),O=i("c31d"),C=i("a142"),w=i("4598"),S=i("7744"),A=i("dfaf"),k=Object(g["a"])("collapse-item"),M=k[0],j=k[1],$=["title","icon","right-icon"],N=M({mixins:[Object(b["a"])("vanCollapse")],props:Object(O["a"])({},A["a"],{name:[Number,String],disabled:Boolean,isLink:{type:Boolean,default:!0}}),data:function(){return{show:null,inited:null}},computed:{currentName:function(){return Object(C["b"])(this.name)?this.name:this.index},expanded:function(){var t=this;if(!this.parent)return null;var e=this.parent,i=e.value,a=e.accordion;return a?i===this.currentName:i.some((function(e){return e===t.currentName}))}},created:function(){this.show=this.expanded,this.inited=this.expanded},watch:{expanded:function(t,e){var i=this;if(null!==e){t&&(this.show=!0,this.inited=!0);var a=t?this.$nextTick:w["b"];a((function(){var e=i.$refs,a=e.content,n=e.wrapper;if(a&&n){var s=a.offsetHeight;if(s){var o=s+"px";n.style.height=t?0:o,Object(w["a"])((function(){n.style.height=t?o:0}))}else i.onTransitionEnd()}}))}}},methods:{onClick:function(){if(!this.disabled){var t=this.parent,e=t.accordion&&this.currentName===t.value?"":this.currentName;this.parent.switch(e,!this.expanded)}},onTransitionEnd:function(){this.expanded?this.$refs.wrapper.style.height="":this.show=!1}},render:function(){var t,e=this,i=arguments[0],a=this.disabled,n=this.expanded,s=$.reduce((function(t,i){return e.slots(i)&&(t[i]=function(){return e.slots(i)}),t}),{});this.slots("value")&&(s.default=function(){return e.slots("value")});var o=i(S["a"],{attrs:{role:"button",tabindex:a?-1:0,"aria-expanded":String(n)},class:j("title",{disabled:a,expanded:n}),on:{click:this.onClick},scopedSlots:s,props:Object(O["a"])({},this.$props)}),r=this.inited&&i("div",{directives:[{name:"show",value:this.show}],ref:"wrapper",class:j("wrapper"),on:{transitionend:this.onTransitionEnd}},[i("div",{ref:"content",class:j("content")},[this.slots()])]);return i("div",{class:[j(),(t={},t[m["c"]]=this.index,t)]},[o,r])}}),L=i("d399"),B=i("6e47"),R=i("16ac"),F=i("8402"),z=i("45bc"),I=(i("fb9c"),i("0147"),i("eeb2"),i("69fa"),i("8d8c"),i("c2d8"),i("e815"),i("5aff"));h["d"].use(f["a"]).use(p["a"]).use(v["a"]).use(T).use(N).use(L["a"]).use(B["a"]);var J=function(t){function e(){var t;return Object(o["a"])(this,e),t=Object(l["a"])(this,Object(d["a"])(e).apply(this,arguments)),t.authTypeText="",t.activeNames=["0"],t.txName="",t.txAddress="",t.allAdminCount=0,t.allowAdminCount=0,t.originAddressList=Array(),t.authOriginalTxModel=new R["b"],t.isShowLoading=!1,t}return Object(c["a"])(e,t),Object(r["a"])(e,[{key:"created",value:function(){this.$store.state.title="授权状态列表",window.nativeSignMsgToJsResult=this.nativeSignMsgToJsResult,this.authOriginalTxModel=this.$store.state.authOriginalTxModel,this.authOriginalTxModel.authType===R["a"].AddAdmin?this.authTypeText=this.$t("authOriginal").toString():this.authTypeText=this.$t("cancelOriginal").toString(),this.txAddress=this.authOriginalTxModel.txAddress,this.txName=this.authOriginalTxModel.txName}},{key:"mounted",value:function(){this.originAddressList=this.authOriginalTxModel.originalAddressModelList,console.log("originAddressList.length=>"+this.authOriginalTxModel.originalAddressModelList.length),this.allAdminCount=this.authOriginalTxModel.originalAddressModelList.length,console.log("this.allAdminCount===>"+this.allAdminCount);var t=0;this.authOriginalTxModel.originalAddressModelList.forEach((function(e){!0===e.signStatus&&(t+=1)})),this.allowAdminCount=t}},{key:"applyForSign",value:function(t){var e=this.authOriginalTxModel.originalAddressModelList,i=e[t];i.signStatus?I["a"].show("该管理员已经完成签名了！~~~"):(console.log("applyForSign model.address"+i.address+t),this.authOriginalTxModel.nowAdminAddress=i.address,this.authOriginalTxModel.nowAdminName=i.name,this.$store.commit("updateTxAdminAddressAndName",this.authOriginalTxModel),this.$router.push({name:"original_sign"}))}},{key:"upTxToChain",value:function(){var t=Object(s["a"])(regeneratorRuntime.mark((function t(){var e,i;return regeneratorRuntime.wrap((function(t){while(1)switch(t.prev=t.next){case 0:if(e=[],this.authOriginalTxModel.originalAddressModelList.forEach((function(t){!0===t.signStatus&&e.push({signInfo:t["signInfo"],accountId:t["address"]})})),!(this.allowAdminCount>=2&&e.length>=2)){t.next=13;break}return console.log("this.$store.state.nativeChainAddress:"+this.$store.state.nativeChainAddress),t.next=6,z["b"].getInstance().callAdminOperation(this.authOriginalTxModel.authType,this.$store.state.nativeChainAddress,this.txAddress,e,this.txName);case 6:if(i=t.sent,""!==i){t.next=10;break}return I["a"].fail("拼装调用链数据失败！"),t.abrupt("return");case 10:F["a"].getInstance().callNativeSignMsgToJs(i),t.next=14;break;case 13:I["a"].show("授权管理员人数不够，现只有"+this.allowAdminCount+"人同意签名。");case 14:case"end":return t.stop()}}),t,this)})));function e(){return t.apply(this,arguments)}return e}()},{key:"nativeSignMsgToJsResult",value:function(t){var e=this;I["a"].show("nativeSignMsgToJsResult===>"+t),console.log("nativeSignMsgToJsResult===>"+t),z["b"].getInstance().sendToChain(t,(function(t){return e.isShowLoading=!1,t.isFinalized&&(console.log("use balance is finished!===>"),I["a"].show("信息上链已完成，可在浏览器查询交易状态")),t.isBroadcast&&(console.log("status.isBroadcast===>"),console.log("use balance is finished!===>"),I["a"].show("信息上链已完成，可在浏览器查询交易状态")),t.isDropped?(console.log("status.isDropped===>"),void I["a"].fail("信息上链失败，操作已被丢弃")):t.isInvalid?(console.log("status.isInvalid!===>"),void I["a"].fail("信息上链失败，无效操作")):void e.$router.push({name:"original_process"})})).catch((function(t){console.log("reason:"+t),I["a"].fail("信息上链失败，错误原因"+t)}))}}]),e}(h["d"]);J=Object(u["a"])([h["a"]],J);var E=J,D=E,q=(i("da1c"),i("2877")),H=Object(q["a"])(D,a,n,!1,null,"f407a684",null);e["default"]=H.exports},"29a4":function(t,e,i){},4598:function(t,e,i){"use strict";(function(t){i.d(e,"b",(function(){return l})),i.d(e,"a",(function(){return d}));var a=i("a142"),n=Date.now();function s(t){var e=Date.now(),i=Math.max(0,16-(e-n)),a=setTimeout(t,i);return n=e+i,a}var o=a["d"]?t:window,r=o.requestAnimationFrame||s;o.cancelAnimationFrame||o.clearTimeout;function l(t){return r.call(o,t)}function d(t){l((function(){l(t)}))}}).call(this,i("c8ba"))},"69fa":function(t,e,i){i("a29f")},"8d8c":function(t,e,i){i("a29f"),i("e60f"),i("29a4")},9884:function(t,e,i){"use strict";i.d(e,"a",(function(){return s})),i.d(e,"b",(function(){return o}));var a=i("2b0e");function n(t){var e=[];function i(t){t.forEach((function(t){e.push(t),t.children&&i(t.children)}))}return i(t),e}function s(t,e){var i,s;void 0===e&&(e={});var o=e.indexKey||"index";return a["a"].extend({inject:(i={},i[t]={default:null},i),computed:(s={parent:function(){return this.disableBindRelation?null:this[t]}},s[o]=function(){return this.bindRelation(),this.parent.children.indexOf(this)},s),mounted:function(){this.bindRelation()},beforeDestroy:function(){var t=this;this.parent&&(this.parent.children=this.parent.children.filter((function(e){return e!==t})))},methods:{bindRelation:function(){if(this.parent&&-1===this.parent.children.indexOf(this)){var t=[].concat(this.parent.children,[this]),e=n(this.parent.slots());t.sort((function(t,i){return e.indexOf(t.$vnode)-e.indexOf(i.$vnode)})),this.parent.children=t}}}})}function o(t){return{provide:function(){var e;return e={},e[t]=this,e},data:function(){return{children:[]}}}}},"9d9e":function(t,e,i){},b650:function(t,e,i){"use strict";var a=i("c31d"),n=i("2638"),s=i.n(n),o=i("d282"),r=i("ba31"),l=i("b1d2"),d=i("48f4"),c=i("ad06"),u=i("543e"),h=Object(o["a"])("button"),f=h[0],p=h[1];function v(t,e,i,a){var n,o=e.tag,h=e.icon,f=e.type,v=e.color,g=e.plain,b=e.disabled,m=e.loading,_=e.hairline,x=e.loadingText,y={};function T(t){m||b||(Object(r["a"])(a,"click",t),Object(d["a"])(a))}function O(t){Object(r["a"])(a,"touchstart",t)}v&&(y.color=g?v:l["f"],g||(y.background=v),-1!==v.indexOf("gradient")?y.border=0:y.borderColor=v);var C=[p([f,e.size,{plain:g,disabled:b,hairline:_,block:e.block,round:e.round,square:e.square}]),(n={},n[l["b"]]=_,n)];function w(){var a,n=[];return m?n.push(t(u["a"],{class:p("loading"),attrs:{size:e.loadingSize,type:e.loadingType,color:"currentColor"}})):h&&n.push(t(c["a"],{attrs:{name:h},class:p("icon")})),a=m?x:i.default?i.default():e.text,a&&n.push(t("span",{class:p("text")},[a])),n}return t(o,s()([{style:y,class:C,attrs:{type:e.nativeType,disabled:b},on:{click:T,touchstart:O}},Object(r["b"])(a)]),[w()])}v.props=Object(a["a"])({},d["c"],{text:String,icon:String,color:String,block:Boolean,plain:Boolean,round:Boolean,square:Boolean,loading:Boolean,hairline:Boolean,disabled:Boolean,nativeType:String,loadingText:String,loadingType:String,tag:{type:String,default:"button"},type:{type:String,default:"default"},size:{type:String,default:"normal"},loadingSize:{type:String,default:"20px"}}),e["a"]=f(v)},da1c:function(t,e,i){"use strict";var a=i("9d9e"),n=i.n(a);n.a},db80:function(t,e,i){},e815:function(t,e,i){i("a29f")},ea47:function(t,e,i){"use strict";var a=i("2638"),n=i.n(a),s=i("d282"),o=i("ba31"),r=i("b1d2"),l=i("7744"),d=i("34e96"),c=Object(s["a"])("panel"),u=c[0],h=c[1];function f(t,e,i,a){var s=function(){return[i.header?i.header():t(l["a"],{attrs:{icon:e.icon,label:e.desc,title:e.title,value:e.status,valueClass:h("header-value")},class:h("header")}),t("div",{class:h("content")},[i.default&&i.default()]),i.footer&&t("div",{class:[h("footer"),r["c"]]},[i.footer()])]};return t(d["a"],n()([{class:h(),scopedSlots:{default:s}},Object(o["b"])(a,!0)]))}f.props={icon:String,desc:String,title:String,status:String},e["a"]=u(f)},fb9c:function(t,e,i){i("a29f"),i("e60f"),i("db80")}}]);
//# sourceMappingURL=chunk-68da2476.f804752e.js.map