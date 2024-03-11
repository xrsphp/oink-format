# drrq

    能自动取消重复请求的axios封装( duplicate removal request)

### 简介

-   开箱即用
-   当请求队列中存在同url和参数的请求时，可拦截重复请求（拦截后面的），并将请求结果共享给所有请求源
-   支持 get/post 等多种请求方式，支持文件上传，支持自定义 headers
-   可自定义是否允许重复请求
-   可基于此代码，二开支持请求日志

#### 安装

    npm i drrq  ||  cnpm i drrq  ||  pnpm i drrq

#### 使用

```js
	import drrq from 'drrq';

	// get
	drrq.get(url, data);

	// post
	drrq.post(url, data);

	// post 请求不去重
	drrq.post(url, data, false);

	// file
	drrq.file(url, data);

	// 完整写法  请求方式，url，参数，请求头，是否去重，是否为文件上传
	drrq.request(method, url, params, headers, preventRepeat = true, uploadFile = false);
```
