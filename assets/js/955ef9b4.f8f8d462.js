"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[5141],{3905:(e,t,n)=>{n.r(t),n.d(t,{MDXContext:()=>p,MDXProvider:()=>m,mdx:()=>f,useMDXComponents:()=>d,withMDXComponents:()=>c});var i=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function r(){return r=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var i in n)Object.prototype.hasOwnProperty.call(n,i)&&(e[i]=n[i])}return e},r.apply(this,arguments)}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);t&&(i=i.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,i)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,i,a=function(e,t){if(null==e)return{};var n,i,a={},r=Object.keys(e);for(i=0;i<r.length;i++)n=r[i],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(i=0;i<r.length;i++)n=r[i],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var p=i.createContext({}),c=function(e){return function(t){var n=d(t.components);return i.createElement(e,r({},t,{components:n}))}},d=function(e){var t=i.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},m=function(e){var t=d(e.components);return i.createElement(p.Provider,{value:t},e.children)},u="mdxType",y={inlineCode:"code",wrapper:function(e){var t=e.children;return i.createElement(i.Fragment,{},t)}},b=i.forwardRef((function(e,t){var n=e.components,a=e.mdxType,r=e.originalType,o=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),c=d(n),m=a,u=c["".concat(o,".").concat(m)]||c[m]||y[m]||r;return n?i.createElement(u,l(l({ref:t},p),{},{components:n})):i.createElement(u,l({ref:t},p))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var r=n.length,o=new Array(r);o[0]=b;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[u]="string"==typeof e?e:a,o[1]=l;for(var p=2;p<r;p++)o[p]=n[p];return i.createElement.apply(null,o)}return i.createElement.apply(null,n)}b.displayName="MDXCreateElement"},37779:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>d,frontMatter:()=>r,metadata:()=>l,toc:()=>p});var i=n(87462),a=(n(67294),n(3905));const r={id:"visibility",title:"Visibility"},o=void 0,l={unversionedId:"concepts/visibility",id:"concepts/visibility",title:"Visibility",description:"Visibility determines whether a target can reference",source:"@site/../docs/concepts/visibility.md",sourceDirName:"concepts",slug:"/concepts/visibility",permalink:"/docs/concepts/visibility",draft:!1,tags:[],version:"current",frontMatter:{id:"visibility",title:"Visibility"},sidebar:"manualSidebar",previous:{title:"Target Pattern",permalink:"/docs/concepts/target_pattern"},next:{title:"Daemon (buckd)",permalink:"/docs/concepts/daemon"}},s={},p=[{value:"Examples",id:"examples",level:2}],c={toc:p};function d(e){let{components:t,...n}=e;return(0,a.mdx)("wrapper",(0,i.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,a.mdx)("p",null,"Visibility determines whether a ",(0,a.mdx)("a",{parentName:"p",href:"/docs/concepts/glossary#target"},"target")," can reference\nanother target in its ",(0,a.mdx)("a",{parentName:"p",href:"/docs/concepts/glossary#attribute"},"attributes"),". In a large project,\nyou may want to prevent developers from 'reaching across' the project and\npulling in additional code. Reducing the visibility of targets can help prevent\nthat type of behavior."),(0,a.mdx)("p",null,"There are two types of visibility attributes available (each of which takes a\nlist of ",(0,a.mdx)("a",{parentName:"p",href:"/docs/concepts/glossary#target-pattern"},"target patterns"),"):"),(0,a.mdx)("ul",null,(0,a.mdx)("li",{parentName:"ul"},(0,a.mdx)("inlineCode",{parentName:"li"},"visibility")," - determines which other targets can depend on a target."),(0,a.mdx)("li",{parentName:"ul"},(0,a.mdx)("inlineCode",{parentName:"li"},"within_view")," - determines which other targets a target can depend on.")),(0,a.mdx)("p",null,"Both attributes act as allowlists, with some exceptions. In general, if a target\nis not listed, there may be no dependency relationship. If the ",(0,a.mdx)("inlineCode",{parentName:"p"},"within_view"),"\nlist is empty or unset, however, its check is bypassed. Similarly, targets\ndefined in the same ",(0,a.mdx)("a",{parentName:"p",href:"/docs/concepts/glossary#buck-file"},"BUCK file")," always act as if they\nwere members of their siblings' ",(0,a.mdx)("inlineCode",{parentName:"p"},"visibility")," lists."),(0,a.mdx)("p",null,"There is also a special value for ",(0,a.mdx)("inlineCode",{parentName:"p"},"visibility")," attribute: ",(0,a.mdx)("inlineCode",{parentName:"p"},"'PUBLIC'"),", which\nmakes a build rule visible to all targets."),(0,a.mdx)("p",null,"In case of logically-conflicting lists, ",(0,a.mdx)("inlineCode",{parentName:"p"},"within_view")," takes precedence over\n",(0,a.mdx)("inlineCode",{parentName:"p"},"visibility"),". If ",(0,a.mdx)("inlineCode",{parentName:"p"},"//foo:bar")," defines ",(0,a.mdx)("inlineCode",{parentName:"p"},"//hello:world")," in its ",(0,a.mdx)("inlineCode",{parentName:"p"},"visibility")," list,\nbut ",(0,a.mdx)("inlineCode",{parentName:"p"},"//hello:world")," does not define ",(0,a.mdx)("inlineCode",{parentName:"p"},"//foo:bar")," in its ",(0,a.mdx)("inlineCode",{parentName:"p"},"within_view")," list, then\n",(0,a.mdx)("inlineCode",{parentName:"p"},"//hello:world")," may not depend on ",(0,a.mdx)("inlineCode",{parentName:"p"},"//foo:bar"),"."),(0,a.mdx)("h2",{id:"examples"},"Examples"),(0,a.mdx)("p",null,"A common library like Guava should be able to be included by any build rule:"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-java"},"prebuilt_jar(\n  name = 'guava',\n  binary_jar = 'guava-14.0.1.jar',\n  visibility = ['PUBLIC']\n)\n")),(0,a.mdx)("p",null,"It is common to restrict the visibility of Android resources to the Java code\nthat uses it:"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-java"},"android_resource(\n  name = 'ui_res',\n  res = 'res',\n  package = 'com.example',\n  visibility = ['//java/com/example/ui:ui']\n)\n")),(0,a.mdx)("p",null,"Or it may be simpler to make it visible to the entire directory in case\nadditional build rules are added to ",(0,a.mdx)("inlineCode",{parentName:"p"},"java/com/example/ui/BUCK"),":"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-java"},"android_resource(\n  name = 'ui_res',\n  res = 'res',\n  package = 'com.example',\n  visibility = ['//java/com/example/ui:']\n)\n")),(0,a.mdx)("p",null,"Also, it is common to limit code for testing to be visible only to tests. If you\ndefine all of your Java unit tests in a folder named ",(0,a.mdx)("inlineCode",{parentName:"p"},"javatests/")," in the root of\nyour project, then you could define the following rule to ensure that only build\nrules under ",(0,a.mdx)("inlineCode",{parentName:"p"},"javatests/")," can depend on JUnit:"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-java"},"prebuilt_jar(\n  name = 'junit',\n  binary_jar = 'junit-4.11.jar',\n  visibility = ['//javatests/...']\n)\n")),(0,a.mdx)("p",null,"Finally, restricting the view of a target can be useful for preventing\ndependency creep:"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-java"},"java_library(\n  name = 'example',\n  visibility = ['PUBLIC',],\n  within_view = ['//foo:bar','//hello:world']\n)\n")))}d.isMDXComponent=!0}}]);