import drrq from '../dist/drrq.js';

const baseURL = 'https://xxxx.cn';

// 处理请求数据  （拼接url，data添加token等） 请根据实际情况调整
const paramsHandler = (url, data) => {
    url = baseURL + url;
    data.token = 'xxxx';
    return { url, data };
};

// 处理全局接口返回的全局处理相关逻辑  请根据实际情况调整
const resHandler = (res) => {
    // TODO 未授权跳转登录，状态码异常报错等
    return res;
};

export const request = async (method, _url, _data = {}, preventRepeat = true) => {
    let { url, data } = paramsHandler(_url, _data);
    let res = null;
    if (method == 'get' || method == 'GET' || method == 'Get') {
        res = await drrq.get(url, data, preventRepeat);
    }
    if (method == 'post' || method == 'POST' || method == 'Post') {
        res = await drrq.post(url, data, preventRepeat);
    }
    if (method == 'file' || method == 'FILE' || method == 'file') {
        res = await drrq.file(url, data, preventRepeat);
    }
    return resHandler(res);
};
