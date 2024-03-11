
import { exampleRequestGet } from './api.js';
const example = async () => {
    let res = await exampleRequestGet();
    console.log('请求成功 ');
};
example();

