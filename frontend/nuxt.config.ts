// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({

    modules: [
        '@nuxtjs/tailwindcss',
        'nuxt-proxy'
    ],
    
    nitro: {
        devProxy: {
            'proxy/get_docs' : {
                target : 'http://localhost:8080/get_docs',
                changeOrigin : true
            }
        }
    },

    proxy: {
        options: {
          target: 'http://localhost:8080',
          changeOrigin: true,
          pathRewrite: {
            '^/api/get_docs': '/get_docs',
            '^/api/users': '/users'
          },
          pathFilter: [
            '/api/get_docs',
            '/api/users'
          ]
        }
    },

})
