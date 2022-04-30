Page({

  data: {
    wallet: {
      public_address: "",
      public_address_qrcode: "",
      public_key: "",
      private_key: ""
    }
  },

  onShow: function (options) {
    const wallet = ft.generate_wallet();
    const qrcode_svg = "data:image/svg+xml;base64," + wallet.public_address_qrcode;
    console.log("qrcode");
    console.log(qrcode_svg);

    this.setData({ 
       ['wallet.public_address']: wallet.public_address,
       ['wallet.public_address_qrcode']: qrcode_svg,
       ['wallet.public_key']: wallet.public_key,
       ['wallet.private_key']: wallet.private_key
    })
  },

  onHide: function () {
    console.log("generate page unload"); 
    ft.release_wallet();
  },

  save_wallet: function() {
    console.log("save wallet");
    // ft.save_wallet();  // 未实现

    ft.showModal({
      title: '提示',
      content: 'Not yet implemented',
      showCancel: false,
      success (res) {
        if (res.confirm) {
          console.log('ok')
        } 
      }
    })
  }

})


