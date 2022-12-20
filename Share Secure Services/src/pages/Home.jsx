//Name : Prem Atul Jethwa
//UTA ID : 1001861810
//Home page which can be seen once user is logged in. It is basically a private chat window.

import React from 'react'
import Sidebar from '../components/Sidebar'
import Chat from '../components/Chat'

const Home = () => {
  return (
    <div className='home'>
      <div className="container">
        <Sidebar/>
        <Chat/>
      </div>
    </div>
  )
}

export default Home