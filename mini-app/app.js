// app.js
App({
  onLaunch() {
    
    const logs = ft.getStorageSync('logs') || []
    logs.unshift(Date.now())
    ft.setStorageSync('logs', logs)
  }

})
