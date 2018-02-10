<!doctype HTML>
<html>
<head>
<link rel="stylesheet" type="text/css" href="/res/skeleton.css"/>
<link rel="stylesheet" type="text/css" href="/res/main.css"/>
<link rel="stylesheet" type="text/css" href="/res/board.css"/>
<script type="text/javascript" src="/res/main.js"></script>
<meta charset="UTF-8"> 
</head>
<body onload="fill_value_from_cookie('fillme', 'password')">
<div class="u-center"><h1>{{payload.board}}</h1></div>
<div class="container">
 <div class="offset-by-one-plus nine columns rules">
 <div>
 <div class="twelve columns button" onclick="toggle_uncles(this);">Rules</div>
 </div>
 <div>
 <div class="offset-by-one eight columns">
  {{#payload.rules}}
  <li>{{.}}</li>
  {{/payload.rules}}
 </div>
 </div>
 </div>
</div>
<div class="container spacer"></div>
  <div>
  <div class="container">
  <div class="button button-primary twelve columns" onclick="toggle_uncles(this);">Stickies</div>
  </div>
  <div class="container">
  <div class="labels">
   <div class="three columns">Thread</div>
   <div class="three columns">Created</div>
   <div class="three columns">Latest post</div>
   <div class="three columns">Post count</div>
  </div>
  <div class="container small-spacer"></div>
 {{#payload.stickies}}
  <div>
    <div class="three columns"><a href="/{{payload.link}}/t/{{uid}}/0">{{title}}</a></div>
    <div class="three columns">{{cdate}}</div>
    <div class="three columns">{{mdate}}</div>
    <div class="three columns">{{post_count}}</div>
  </div>
   {{/payload.stickies}}
   </div>
 <div class="container spacer"></div>
 </div>
<div class="container spacer"></div>
  <div>
  <div class="container">
  <div class="button button-primary twelve columns" onclick="toggle_uncles(this);">Active Threads</div>
  </div>
  <div class="container">
  <div class="labels">
   <div class="three columns">Thread</div>
   <div class="three columns">Created</div>
   <div class="three columns">Latest post</div>
   <div class="three columns">Post count</div>
  </div>
  <div class="container small-spacer"></div>
 {{#payload.threads}}
  <div>
    <div class="three columns"><a href="/{{payload.link}}/t/{{uid}}/0">{{title}}</a></div>
    <div class="three columns">{{cdate}}</div>
    <div class="three columns">{{mdate}}</div>
    <div class="three columns">{{post_count}}</div>
  </div>
   {{/payload.threads}}
 <div class="pages-area">
<div class="small">Jump to page</div><div class="pages">
{{#payload.pages}}
{{#link}}<a href="{{num}}">{{/link}}{{num}}{{#link}}</a>{{/link}}
{{/payload.pages}}
</div>
   </div>
   </div>
 <div class="container spacer"></div>
 </div>
<div class="container spacer"></div>

<form method="post" action="/{{payload.link}}/new">
 <div class="container post-form">
 <div class="container spacer"></div>
 <div class="container">
 <label for="title">Title</label>
 <input type="text" name="title" alt="Thread title.">
 <label for="name">Name</label>
 <input type="text" name="name" value="Anonymous" alt="Name to post as. Max 64 characters." maxlength=64>
 <label for="password">Password</label>
 <input type="text" name="password" class="fillme" id="thread_pwd" alt="Password for post deletion.">
 </div>
 <div class="container">
 <label for="content">Post</label>
 <textarea name="content"></textarea>
 </div>
 <div class="container">
 <button type="submit" onclick="save_password_cookie('thread_pwd')">Make Thread</button>
 </div>
 </div>
</form>
</body>
</html>
