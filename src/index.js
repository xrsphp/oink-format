import qs from "qs";
import axios from "axios";

let pending = []; //用于存储每个ajax请求的取消函数和ajax标识
let task = {}; //用于存储每个ajax请求的处理函数，通过请求结果调用，以ajax标识为key

//请求开始前推入pending
const pushPending = (item) => {
    pending.push(item);
};
//请求完成后取消该请求，从列表删除
const removePending = (key) => {
    for (let p in pending) {
        if (pending[p].key === key) {
            //当前请求在列表中存在时
            pending[p].cancelToken(); //执行取消操作
            pending.splice(p, 1); //把这条记录从列表中移除
        }
    }
};
//请求前判断是否已存在该请求
const existInPending = (key) => {
    return pending.some((e) => e.key === key);
};

// 创建task
const createTask = (key, resolve) => {
    let callback = (response) => {
        resolve(response.data);
    };
    if (!task[key]) task[key] = [];
    task[key].push(callback);
};
// 处理task
const handleTask = (key, response) => {
    for (let i = 0; task[key] && i < task[key].length; i++) {
        task[key][i](response);
    }
    task[key] = undefined;
};

const getHeaders = { 'Content-Type': 'application/json' };
const postHeaders = { 'Content-Type': 'application/x-www-form-urlencoded' };
const fileHeaders = { 'Content-Type': 'multipart/form-data' };

const request = (method, url, params, headers, preventRepeat = true, uploadFile = false) => {
    let key = url + '?' + qs.stringify(params);
    return new Promise((resolve, reject) => {
        const instance = axios.create({
            baseURL: url,
            headers,
            timeout: 30 * 1000,
        });

        instance.interceptors.request.use(
            (config) => {
                if (preventRepeat) {
                    config.cancelToken = new axios.CancelToken((cancelToken) => {
                        // 判断是否存在请求中的当前请求 如果有取消当前请求
                        if (existInPending(key)) {
                            cancelToken();
                        } else {
                            pushPending({ key, cancelToken });
                        }
                    });
                }
                return config;
            },
            (err) => {
                return Promise.reject(err);
            }
        );

        instance.interceptors.response.use(
            (response) => {
                if (preventRepeat) {
                    removePending(key);
                }
                return response;
            },
            (error) => {
                return Promise.reject(error);
            }
        );

        // 请求执行前加入task
        createTask(key, resolve);

        instance(Object.assign({}, { method }, method === 'post' || method === 'put' ? { data: !uploadFile ? qs.stringify(params) : params } : { params }))
            .then((response) => {
                // 处理task
                handleTask(key, response);
            })
            .catch(() => {});
    });
};

export const get = (url, data = {}, preventRepeat = true) => {
    return request('get', url, data, getHeaders, preventRepeat, false);
};
 export const post = (url, data = {}, preventRepeat = true) => {
     return request('post', url, data, postHeaders, preventRepeat, false);
 };
 export const file = (url, data = {}, preventRepeat = true) => {
     return request('post', url, data, fileHeaders, preventRepeat, true);
 };
export default { request, get, post, file };