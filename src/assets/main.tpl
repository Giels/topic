<!doctype HTML>
<html>
<head>
<link rel="stylesheet" type="text/css" href="/res/skeleton.css"/>
<link rel="stylesheet" type="text/css" href="/res/main.css"/>
<script type="text/javascript" src="/res/main.js"></script>
<meta charset="UTF-8"> 
</head>
<body>
<div class="u-center"><h1>{{payload.name}}</h1></div>
<div class="container">
 {{#payload.cats}}
  <div>
  <div class="container">
  <div class="button button-primary twelve columns" onclick="toggle_uncles(this);">{{name}}</div>
  </div>
  <div class="container">
  <div class="labels">
   <div class="five columns">Board</div>
   <div class="five columns">Active threads</div>
  </div>
  <div class="container small-spacer"></div>
  {{#boards}}
  <div>
   <div class="five columns"><a href="{{link}}/0">{{name}}</a></div>
   <div class="five columns">{{thread_count}}</div>
  </div>
  {{/boards}}
  </div>
  <div class="container spacer"></div>
  </div>
 {{/payload.cats}}
</div>
</body>
</html>
