module.exports = {
  extApi:[
    { 
      name: 'generate_wallet', 
      sync: true, //是否为同步api
      params: { //扩展api 的参数格式，可以只列必须的属性
      }
    },
    { 
      name: 'release_wallet', 
      sync: true, 
      params: { 
      }
    },
    { 
      name: 'save_wallet', 
      sync: true, 
      params: { 
      }
    },
    { 
      name: 'fetch_wallet', 
      sync: true, 
      params: { 
      }
    }
  ]
}