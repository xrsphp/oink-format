
import { request } from './request.js';
// 示例请求Get
export const exampleRequestGet = (data) => request('get', '/xxxx', data);

// 示例请求Post
export const exampleRequestPost = (data) => request('post', '/xxxx', data);

// 示例请求Post 不去重
export const exampleRequestPost2 = (data) => request('post', '/xxxx', data, false);

// 示例请求Post 不去重
export const exampleRequestFile = (data) => request('file', '/xxxx', data, false);