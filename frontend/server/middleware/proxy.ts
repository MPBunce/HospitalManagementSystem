import { defineEventHandler } from 'h3'
import { createProxyMiddleware } from 'http-proxy-middleware'; // npm install http-proxy-middleware@beta

const apiProxyMiddleware = createProxyMiddleware({
    target: 'http://127.0.0.1:8080/',
    changeOrigin: true,
    ws: true,
    pathRewrite: {
      '^/api/todos': '/todos',
      '^/api/users': '/users'
    },
    pathFilter: [
      '/api/todos',
      '/api/users'
    ],
    logger: console
})

export default defineEventHandler(async (event) => {
  await new Promise((resolve, reject) => {
    const next = (err?: unknown) => {
      if (err) {
        reject(err)
      } else {
        resolve(true)
      }
    }

    apiProxyMiddleware(event.req, event.res, next)
  })
})