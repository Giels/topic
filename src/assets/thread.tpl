<!doctype HTML>
<html>
<head>
<link rel="stylesheet" type="text/css" href="/res/skeleton.css"/>
<link rel="stylesheet" type="text/css" href="/res/main.css"/>
<link rel="stylesheet" type="text/css" href="/res/board.css"/>
<link rel="stylesheet" type="text/css" href="/res/thread.css"/>
<script type="text/javascript" src="/res/main.js"></script>
<meta charset="UTF-8"> 
</head>

<body onload="fill_value_from_cookie('.fillme', 'password'); attach_image_click_callback();">
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
<div class="container">
<form method="post" action="/{{payload.here}}/report_delete">
 {{#payload.posts}}
  <div>
  <div class="container">
  <a name="{{number}}"></a>
  <div class="twelve columns button button-primary{{#bump}}-bump{{/bump}} post-type-{{typ}}">
  <div class="one column">
  <input type="checkbox" name="p_{{uid}}">
  </div>
  <div class="post-button eleven columns" onclick="toggle_uncles(this.parentElement);"><div class="left">{{number}}</div><div class="right">{{name}} - {{date}}</div></div></div>
  </div>
  <div class="container">
   <div class="twelve columns content">{{{content}}}</div>
  </div>
  <div class="container small-spacer"></div>
  </div>
 {{/payload.posts}}
 <div class="pages-area">
<div class="small">Jump to page</div><div class="pages">
{{#payload.pages}}
{{#link}}<a href="{{num}}">{{/link}}{{num}}{{#link}}</a>{{/link}}
{{/payload.pages}}
</div>
   </div>
   </div>
 <div class="container spacer"></div>
 <div class="rd-form">
 <div class="container">
  <label for="password">Deletion Password</label>
  <input type="text" name="password" id="delete_pwd">
  <button type="submit" name="action" value="delete">Delete</button>
 </div>
 <div class="container">
  <label for="reason">Report Reason</label>
  <input type="text" name="reason" maxlength=64 alt="Reason for the report. Max 64 characters."></label>
  <button type="submit" name="action" value="report">Report</button>
 </div>
 </div>
 </form>
</div>
<div class="container spacer"></div>
<form method="post" action="/{{payload.here}}/new">
<div class="post-form">
 <div class="container">
 <label for="name">Name</label>
 <input type="text" name="name" value="Anonymous" maxlength=64 alt="Name to post with. Max 64 characters.">
 <label for="password">Password</label>
 <input type="text" name="password" id="post_pwd" class="fillme" alt="Password for post deletion.">
 <label for="bump">Bump</label>
 <input type="checkbox" name="bump" alt="Whether to bump the thread.">
 </div>
 <div class="container">
 <label for="content">Post</label>
 <textarea name="content"></textarea>
 </div>
 <div class="container">
 <button type="submit" onclick="save_password_cookie('post_pwd');">Post</button>
 </div>
 </div>
</form>
</body>
</html>
