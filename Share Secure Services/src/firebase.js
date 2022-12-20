//Name : Prem Atul Jethwa
//UTA ID : 1001861810

// Import the functions you need from the SDKs you need
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration

import { initializeApp } from "firebase/app";
import { getAuth } from "firebase/auth";
import { getStorage } from "firebase/storage";
import { getFirestore } from "firebase/firestore";

const firebaseConfig = {
  apiKey: "AIzaSyBsEKRuTv1MEotb0ruGHXeW1BzaK23dHLg",
  authDomain: "sp-assignment-3.firebaseapp.com",
  projectId: "sp-assignment-3",
  storageBucket: "sp-assignment-3.appspot.com",
  messagingSenderId: "1081150297579",
  appId: "1:1081150297579:web:f1322c21ad5c74d548b5ae"
};

// Initialize Firebase
export const app = initializeApp(firebaseConfig);
export const auth = getAuth();
export const storage = getStorage();
export const db = getFirestore()
