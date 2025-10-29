<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.11" tiledversion="1.11.2" name="grass_biome" tilewidth="16" tileheight="16" tilecount="252" columns="12">
 <image source="overworld_tileset_grass.png" width="192" height="336"/>
 <tile id="0" probability="20"/>
 <tile id="48">
  <objectgroup draworder="index" id="2">
   <object id="1" x="6.09091" y="8.09091" width="9.90909" height="7.90909"/>
  </objectgroup>
 </tile>
 <tile id="49">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="7.09091" width="16" height="8.90909"/>
  </objectgroup>
 </tile>
 <tile id="50">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="8.27273" width="10.0909" height="7.72727"/>
  </objectgroup>
 </tile>
 <tile id="60">
  <objectgroup draworder="index" id="2">
   <object id="1" x="6.63636" y="0" width="9.36364" height="16"/>
  </objectgroup>
 </tile>
 <tile id="61">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="16" height="16"/>
  </objectgroup>
 </tile>
 <tile id="62">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="9.90909" height="16"/>
  </objectgroup>
 </tile>
 <tile id="72">
  <objectgroup draworder="index" id="2">
   <object id="1" x="6" y="0" width="10" height="11.6364"/>
  </objectgroup>
 </tile>
 <tile id="73">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="16" height="11.3636"/>
  </objectgroup>
 </tile>
 <tile id="74">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="11" height="10.7273"/>
  </objectgroup>
 </tile>
 <tile id="187">
  <objectgroup draworder="index" id="2">
   <object id="1" x="17.8182" y="3.72727">
    <ellipse/>
   </object>
  </objectgroup>
 </tile>
 <tile id="188">
  <objectgroup draworder="index" id="2">
   <object id="1" x="2.45455" y="3.45455">
    <ellipse/>
   </object>
  </objectgroup>
 </tile>
 <tile id="197" type="well">
  <objectgroup draworder="index" id="3">
   <object id="2" x="2.69565" y="2.52174" width="10.7175" height="11.9565">
    <ellipse/>
   </object>
  </objectgroup>
 </tile>
 <wangsets>
  <wangset name="Terrains" type="corner" tile="-1">
   <wangcolor name="Grass" color="#ff0000" tile="13" probability="1"/>
   <wangcolor name="Water" color="#00ff00" tile="0" probability="1"/>
   <wangcolor name="Forest" color="#0000ff" tile="0" probability="1"/>
   <wangcolor name="Swampgrass" color="#ff7700" tile="0" probability="1"/>
   <wangcolor name="Swamp" color="#00e9ff" tile="0" probability="1"/>
   <wangtile tileid="0" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="1" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="2" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="3" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="4" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="13" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="14" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="25" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="26" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="37" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="38" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="48" wangid="0,1,0,2,0,1,0,1"/>
   <wangtile tileid="49" wangid="0,1,0,2,0,2,0,1"/>
   <wangtile tileid="50" wangid="0,1,0,1,0,2,0,1"/>
   <wangtile tileid="51" wangid="0,4,0,5,0,4,0,4"/>
   <wangtile tileid="52" wangid="0,4,0,5,0,5,0,4"/>
   <wangtile tileid="53" wangid="0,4,0,4,0,5,0,4"/>
   <wangtile tileid="57" wangid="0,1,0,3,0,1,0,1"/>
   <wangtile tileid="58" wangid="0,1,0,3,0,3,0,1"/>
   <wangtile tileid="59" wangid="0,1,0,1,0,3,0,1"/>
   <wangtile tileid="60" wangid="0,2,0,2,0,1,0,1"/>
   <wangtile tileid="61" wangid="0,2,0,2,0,2,0,2"/>
   <wangtile tileid="62" wangid="0,1,0,1,0,2,0,2"/>
   <wangtile tileid="63" wangid="0,5,0,5,0,4,0,4"/>
   <wangtile tileid="64" wangid="0,5,0,5,0,5,0,5"/>
   <wangtile tileid="65" wangid="0,4,0,4,0,5,0,5"/>
   <wangtile tileid="69" wangid="0,3,0,3,0,1,0,1"/>
   <wangtile tileid="70" wangid="0,3,0,3,0,3,0,3"/>
   <wangtile tileid="71" wangid="0,1,0,1,0,3,0,3"/>
   <wangtile tileid="72" wangid="0,2,0,1,0,1,0,1"/>
   <wangtile tileid="73" wangid="0,2,0,1,0,1,0,2"/>
   <wangtile tileid="74" wangid="0,1,0,1,0,1,0,2"/>
   <wangtile tileid="75" wangid="0,5,0,4,0,4,0,4"/>
   <wangtile tileid="76" wangid="0,5,0,4,0,4,0,5"/>
   <wangtile tileid="77" wangid="0,4,0,4,0,4,0,5"/>
   <wangtile tileid="81" wangid="0,3,0,1,0,1,0,1"/>
   <wangtile tileid="82" wangid="0,3,0,1,0,1,0,3"/>
   <wangtile tileid="83" wangid="0,1,0,1,0,1,0,3"/>
   <wangtile tileid="84" wangid="0,2,0,1,0,2,0,2"/>
   <wangtile tileid="85" wangid="0,2,0,2,0,1,0,2"/>
   <wangtile tileid="87" wangid="0,5,0,4,0,5,0,5"/>
   <wangtile tileid="88" wangid="0,5,0,5,0,4,0,5"/>
   <wangtile tileid="89" wangid="0,4,0,4,0,4,0,4"/>
   <wangtile tileid="93" wangid="0,3,0,1,0,3,0,3"/>
   <wangtile tileid="94" wangid="0,3,0,3,0,1,0,3"/>
   <wangtile tileid="96" wangid="0,1,0,2,0,2,0,2"/>
   <wangtile tileid="97" wangid="0,2,0,2,0,2,0,1"/>
   <wangtile tileid="99" wangid="0,4,0,5,0,5,0,5"/>
   <wangtile tileid="100" wangid="0,5,0,5,0,5,0,4"/>
   <wangtile tileid="105" wangid="0,1,0,3,0,3,0,3"/>
   <wangtile tileid="106" wangid="0,3,0,3,0,3,0,1"/>
   <wangtile tileid="123" wangid="0,4,0,4,0,4,0,4"/>
   <wangtile tileid="124" wangid="0,4,0,4,0,4,0,4"/>
   <wangtile tileid="125" wangid="0,4,0,4,0,4,0,4"/>
   <wangtile tileid="135" wangid="0,4,0,4,0,4,0,4"/>
   <wangtile tileid="136" wangid="0,4,0,4,0,4,0,4"/>
   <wangtile tileid="137" wangid="0,4,0,4,0,4,0,4"/>
  </wangset>
 </wangsets>
</tileset>
