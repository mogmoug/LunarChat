import { createHmac } from "crypto"
const LATENCY = 16

export function createMessage({author,text,group},cb){
    const timestamp = Date.now()
    const id = 'm_' + timestamp + createHmac('sha256',text).digest('hex').toString();
    const message = {
      id,
      text,
      timestamp,
      groupID: group.id,
      groupName: group.name,
      authorName: author.name
    }
    setTimeout(function () {
      cb(message)
    }, LATENCY)
  
}