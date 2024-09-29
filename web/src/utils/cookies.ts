export const setCookie = (name: string, value: unknown, exp = 1) => {
    const date = new Date()
    date.setTime(date.getTime() + (exp * 24 * 60 * 60 * 1000))
    const expires = 'expires=' + date.toUTCString()
    document.cookie = `${name}=${value}; ${expires}; path=/`
  }
  
  export const deleteCookie = (name: string) => {
    const exp = 'expires= Thu, 01 Jan 1970 00:00:00 UTC'
    document.cookie = `${name}=; ${exp}; path=/`
  }
  
  export const getCookie = (name: string) => {
    const cName = name + '='
    const cDecode = decodeURIComponent(document.cookie)
    const cArr = cDecode.split('; ')

    let res: string | undefined
    
    cArr.forEach(r => {
        if (r.indexOf(cName) === 0) res = r.substring(cName.length)
    })
  
    return res
  }