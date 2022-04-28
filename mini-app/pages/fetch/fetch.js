// index.js
//const util = require('../../utils/util.js')

Page({
  data: {
    wallet: {
      public_address: "???",
      public_key: "???",
      private_key: "???"   
    }
  },

  onLoad() {
    console.log("fetch wallet");
   
    // === 暂未实现 ===
    // const wallet = ft.fetch_wallet();
    // console.log(wallet);
    // this.setData({
    //    ['wallet.public_address']: wallet.public_address,
    //    ['wallet.public_key']: wallet.public_key,
    //    ['wallet.private_key']: wallet.private_key
    // })
  },

  onUnload() {
    console.log("fetch page unload");
    // ft.release_wallet();
  },

  goto_generate: function() {
    console.log("go to generate wallet");
    ft.navigateTo({
      url: '/pages/generate/generate'
    });
      
  },

  fetch_wallet: function() {
    const fs = ft.getFileSystemManager();
    try {
      const res = fs.readFileSync(`${ft.env.USER_DATA_PATH}/wallet.json`, 'utf8', 0);
      console.log(res);
      this.setData({
          wallet: res
       })
    } catch(e) {
        console.error(e)
    }
  }
})
