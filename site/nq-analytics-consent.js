(function(){
  var KEY='nq-analytics-consent';
  var choice=null;
  try{choice=localStorage.getItem(KEY);}catch(e){}
  function save(value){try{localStorage.setItem(KEY,value);}catch(e){}}
  function setConsent(value){
    save(value);
    if(value==='granted'&&typeof window.nqLoadGTM==='function')window.nqLoadGTM();
    if(value==='denied'&&window.gtag)window.gtag('consent','update',{analytics_storage:'denied',ad_storage:'denied',ad_user_data:'denied',ad_personalization:'denied'});
    if(banner)banner.hidden=true;
  }
  var banner=document.createElement('section');
  banner.id='nq-consent-banner';banner.setAttribute('role','dialog');banner.setAttribute('aria-label','Analytics consent');banner.setAttribute('aria-describedby','nq-consent-copy');banner.hidden=true;
  banner.innerHTML='<p id="nq-consent-copy"><strong>NEXUS-Q analytics.</strong> We use Google Analytics to understand aggregate website usage and improve the site. Analytics is disabled unless you choose to accept. You can change your choice at any time. <a href="/privacy">Privacy</a> · <a href="/cookies">Cookies</a></p><div class="nq-consent-actions"><button type="button" data-choice="accept">Accept analytics</button><button type="button" data-choice="reject">Reject analytics</button></div>';
  document.addEventListener('DOMContentLoaded',function(){
    document.body.appendChild(banner);
    var prefs=document.createElement('button');prefs.id='nq-consent-preferences';prefs.type='button';prefs.textContent='Privacy preferences';prefs.setAttribute('aria-label','Change analytics consent preferences');document.body.appendChild(prefs);
    banner.addEventListener('click',function(e){var b=e.target.closest('button[data-choice]');if(b)setConsent(b.dataset.choice==='accept'?'granted':'denied');});
    prefs.addEventListener('click',function(){banner.hidden=false;});
    if(choice!=='granted'&&choice!=='denied')banner.hidden=false;
  });
})();
