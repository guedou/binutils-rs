var rot13map;

function rot13(elt)
{
  try
	{
	 var a = elt.value;

   if (!rot13map)
	 {
    rot13map= new Array();
    var s   = "abcdefghijklmnopqrstuvwxyz";
  
    for (i=0; i<s.length; i++)
      rot13map[s.charAt(i)]			= s.charAt((i+13)%26);
    for (i=0; i<s.length; i++)
      rot13map[s.charAt(i).toUpperCase()]	= s.charAt((i+13)%26).toUpperCase();
	 }
   var s = "";
   for (var i=0; i<a.length; i++)
    {
      var b = a.charAt(i);

      s	+= ((b>='A' && b<='Z' || b>='a' && b<='z') ? rot13map[b] : b);
    }
    elt.value=s;
	}
	catch(e)
	{
	  // failed. Fallback to server.
	  return true;
	}
  return false;
}
