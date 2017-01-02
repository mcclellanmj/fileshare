'use strict';
require('./index.html');
var sheet = require('./stylesheet/Stylesheets');
var Elm = require('./src/Main');

Elm.Main.embed(document.getElementById('main'));