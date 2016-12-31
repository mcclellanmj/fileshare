'use strict';
require('./index.html');
require('./stylesheet/Stylesheets');

var Elm = require('./src/Main');
Elm.Main.embed(document.getElementById('main'));