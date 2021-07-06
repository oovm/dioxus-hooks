## useWindow

A really common need is to get the current size of the browser window.

This hook returns an object containing the window's width and height. 

If executed server-side (no window object) the value of width and height will be regard as `iPhone X`.
