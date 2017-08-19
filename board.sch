<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE eagle SYSTEM "eagle.dtd">
<eagle version="8.3.1">
<drawing>
<settings>
<setting alwaysvectorfont="no"/>
<setting verticaltext="up"/>
</settings>
<grid distance="0.1" unitdist="inch" unit="inch" style="lines" multiple="1" display="no" altdistance="0.01" altunitdist="inch" altunit="inch"/>
<layers>
<layer number="1" name="Top" color="4" fill="1" visible="no" active="no"/>
<layer number="16" name="Bottom" color="1" fill="1" visible="no" active="no"/>
<layer number="17" name="Pads" color="2" fill="1" visible="no" active="no"/>
<layer number="18" name="Vias" color="2" fill="1" visible="no" active="no"/>
<layer number="19" name="Unrouted" color="6" fill="1" visible="no" active="no"/>
<layer number="20" name="Dimension" color="15" fill="1" visible="no" active="no"/>
<layer number="21" name="tPlace" color="7" fill="1" visible="no" active="no"/>
<layer number="22" name="bPlace" color="7" fill="1" visible="no" active="no"/>
<layer number="23" name="tOrigins" color="15" fill="1" visible="no" active="no"/>
<layer number="24" name="bOrigins" color="15" fill="1" visible="no" active="no"/>
<layer number="25" name="tNames" color="7" fill="1" visible="no" active="no"/>
<layer number="26" name="bNames" color="7" fill="1" visible="no" active="no"/>
<layer number="27" name="tValues" color="7" fill="1" visible="no" active="no"/>
<layer number="28" name="bValues" color="7" fill="1" visible="no" active="no"/>
<layer number="29" name="tStop" color="7" fill="3" visible="no" active="no"/>
<layer number="30" name="bStop" color="7" fill="6" visible="no" active="no"/>
<layer number="31" name="tCream" color="7" fill="4" visible="no" active="no"/>
<layer number="32" name="bCream" color="7" fill="5" visible="no" active="no"/>
<layer number="33" name="tFinish" color="6" fill="3" visible="no" active="no"/>
<layer number="34" name="bFinish" color="6" fill="6" visible="no" active="no"/>
<layer number="35" name="tGlue" color="7" fill="4" visible="no" active="no"/>
<layer number="36" name="bGlue" color="7" fill="5" visible="no" active="no"/>
<layer number="37" name="tTest" color="7" fill="1" visible="no" active="no"/>
<layer number="38" name="bTest" color="7" fill="1" visible="no" active="no"/>
<layer number="39" name="tKeepout" color="4" fill="11" visible="no" active="no"/>
<layer number="40" name="bKeepout" color="1" fill="11" visible="no" active="no"/>
<layer number="41" name="tRestrict" color="4" fill="10" visible="no" active="no"/>
<layer number="42" name="bRestrict" color="1" fill="10" visible="no" active="no"/>
<layer number="43" name="vRestrict" color="2" fill="10" visible="no" active="no"/>
<layer number="44" name="Drills" color="7" fill="1" visible="no" active="no"/>
<layer number="45" name="Holes" color="7" fill="1" visible="no" active="no"/>
<layer number="46" name="Milling" color="3" fill="1" visible="no" active="no"/>
<layer number="47" name="Measures" color="7" fill="1" visible="no" active="no"/>
<layer number="48" name="Document" color="7" fill="1" visible="no" active="no"/>
<layer number="49" name="Reference" color="7" fill="1" visible="no" active="no"/>
<layer number="51" name="tDocu" color="7" fill="1" visible="no" active="no"/>
<layer number="52" name="bDocu" color="7" fill="1" visible="no" active="no"/>
<layer number="90" name="Modules" color="5" fill="1" visible="yes" active="yes"/>
<layer number="91" name="Nets" color="2" fill="1" visible="yes" active="yes"/>
<layer number="92" name="Busses" color="1" fill="1" visible="yes" active="yes"/>
<layer number="93" name="Pins" color="2" fill="1" visible="no" active="yes"/>
<layer number="94" name="Symbols" color="4" fill="1" visible="yes" active="yes"/>
<layer number="95" name="Names" color="7" fill="1" visible="yes" active="yes"/>
<layer number="96" name="Values" color="7" fill="1" visible="yes" active="yes"/>
<layer number="97" name="Info" color="7" fill="1" visible="yes" active="yes"/>
<layer number="98" name="Guide" color="6" fill="1" visible="yes" active="yes"/>
<layer number="99" name="SpiceOrder" color="5" fill="1" visible="yes" active="yes"/>
</layers>
<schematic xreflabel="%F%N/%S.%C%R" xrefpart="/%S.%C%R">
<libraries>
<library name="we-switch">
<description>Switches, Keys...</description>
<packages>
<package name="TSWB3NCB111LFS">
<pad name="S4" x="5.2" y="16.16" drill="1.45"/>
<pad name="S2" x="-5.2" y="-16.16" drill="1.45"/>
<hole x="-15" y="0" drill="4"/>
<hole x="15" y="0" drill="4"/>
<hole x="0" y="15" drill="4"/>
<hole x="0" y="-15" drill="4"/>
<pad name="S5" x="-13.97" y="9.26" drill="1.45"/>
<pad name="S3" x="13.97" y="-9.26" drill="1.45"/>
<pad name="COMB" x="-13.67" y="-9.07" drill="1.45"/>
<pad name="COMA" x="0" y="9.26" drill="1.45"/>
<pad name="S1" x="-1.57" y="0" drill="1.45"/>
<pad name="A" x="0" y="-9.26" drill="1.45"/>
<pad name="B" x="9.26" y="0" drill="1.45"/>
<circle x="0" y="0" radius="17.2" width="0.127" layer="21"/>
<circle x="0" y="0" radius="16" width="0.127" layer="21"/>
<circle x="0" y="0" radius="11.45" width="0.127" layer="21"/>
<circle x="0" y="0" radius="4.05" width="0.127" layer="21"/>
<circle x="0" y="0" radius="15.292825" width="0.127" layer="21"/>
<circle x="0" y="0" radius="10.8" width="0.127" layer="21"/>
<polygon width="0.127" layer="21">
<vertex x="-10.16" y="-10.16"/>
<vertex x="-10.16" y="-8.89"/>
<vertex x="-8.89" y="-10.16"/>
</polygon>
<polygon width="0.127" layer="21">
<vertex x="8.89" y="-10.16"/>
<vertex x="10.16" y="-10.16"/>
<vertex x="10.16" y="-8.89"/>
</polygon>
<polygon width="0.127" layer="21">
<vertex x="8.89" y="10.16"/>
<vertex x="10.16" y="10.16"/>
<vertex x="10.16" y="8.89"/>
</polygon>
<polygon width="0.127" layer="21">
<vertex x="-10.16" y="8.89"/>
<vertex x="-8.89" y="10.16"/>
<vertex x="-10.16" y="10.16"/>
</polygon>
<circle x="-10.16" y="0" radius="0.3" width="0.127" layer="21"/>
<circle x="10.16" y="0" radius="0.3" width="0.127" layer="21"/>
<circle x="0" y="10.16" radius="0.3" width="0.127" layer="21"/>
<circle x="0" y="-10.16" radius="0.3" width="0.127" layer="21"/>
<circle x="-3.8880625" y="9.386615625" radius="0.3" width="0.127" layer="21"/>
<circle x="-7.18420625" y="7.18420625" radius="0.3" width="0.127" layer="21"/>
<circle x="-9.386615625" y="3.8880625" radius="0.3" width="0.127" layer="21"/>
<circle x="-9.386615625" y="-3.8880625" radius="0.3" width="0.127" layer="21"/>
<circle x="-7.18420625" y="-7.18420625" radius="0.3" width="0.127" layer="21"/>
<circle x="-3.8880625" y="-9.386615625" radius="0.3" width="0.127" layer="21"/>
<circle x="3.8880625" y="-9.386615625" radius="0.3" width="0.127" layer="21"/>
<circle x="7.18420625" y="-7.18420625" radius="0.3" width="0.127" layer="21"/>
<circle x="9.386615625" y="-3.8880625" radius="0.3" width="0.127" layer="21"/>
<circle x="9.386615625" y="3.8880625" radius="0.3" width="0.127" layer="21"/>
<circle x="7.18420625" y="7.18420625" radius="0.3" width="0.127" layer="21"/>
<circle x="3.8880625" y="9.386615625" radius="0.3" width="0.127" layer="21"/>
<hole x="0" y="4.5" drill="1.6"/>
<hole x="0" y="-4.5" drill="1.6"/>
<text x="0" y="6.35" size="0.8128" layer="25" font="vector" ratio="10" align="bottom-center">&gt;NAME</text>
</package>
</packages>
<symbols>
<symbol name="TSWB3NCB111LFS">
<pin name="S1" x="7.62" y="12.7" visible="off" length="middle" rot="R180"/>
<pin name="A" x="7.62" y="7.62" visible="off" length="middle" rot="R180"/>
<pin name="B" x="7.62" y="2.54" visible="off" length="middle" rot="R180"/>
<pin name="S2" x="7.62" y="-5.08" visible="off" length="middle" rot="R180"/>
<pin name="S3" x="7.62" y="-10.16" visible="off" length="middle" rot="R180"/>
<pin name="S4" x="7.62" y="-15.24" visible="off" length="middle" rot="R180"/>
<pin name="S5" x="7.62" y="-20.32" visible="off" length="middle" rot="R180"/>
<wire x1="-2.54" y1="12.7" x2="2.54" y2="15.24" width="0.254" layer="94"/>
<wire x1="-2.54" y1="7.62" x2="2.54" y2="10.16" width="0.254" layer="94"/>
<wire x1="-2.54" y1="2.54" x2="2.54" y2="5.08" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-5.08" x2="2.54" y2="-2.54" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-10.16" x2="2.54" y2="-7.62" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-15.24" x2="2.54" y2="-12.7" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-20.32" x2="2.54" y2="-17.78" width="0.254" layer="94"/>
<wire x1="-2.54" y1="12.7" x2="-7.62" y2="12.7" width="0.254" layer="94"/>
<wire x1="-7.62" y1="12.7" x2="-7.62" y2="7.62" width="0.254" layer="94"/>
<wire x1="-7.62" y1="7.62" x2="-2.54" y2="7.62" width="0.254" layer="94"/>
<wire x1="-7.62" y1="7.62" x2="-7.62" y2="2.54" width="0.254" layer="94"/>
<wire x1="-7.62" y1="2.54" x2="-2.54" y2="2.54" width="0.254" layer="94"/>
<wire x1="-2.54" y1="-5.08" x2="-7.62" y2="-5.08" width="0.254" layer="94"/>
<wire x1="-7.62" y1="-5.08" x2="-7.62" y2="-10.16" width="0.254" layer="94"/>
<wire x1="-7.62" y1="-10.16" x2="-2.54" y2="-10.16" width="0.254" layer="94"/>
<wire x1="-7.62" y1="-10.16" x2="-7.62" y2="-15.24" width="0.254" layer="94"/>
<wire x1="-7.62" y1="-15.24" x2="-2.54" y2="-15.24" width="0.254" layer="94"/>
<wire x1="-7.62" y1="-15.24" x2="-7.62" y2="-20.32" width="0.254" layer="94"/>
<wire x1="-7.62" y1="-20.32" x2="-2.54" y2="-20.32" width="0.254" layer="94"/>
<text x="-7.62" y="10.16" size="1.778" layer="95" rot="R180">COM A</text>
<text x="-7.62" y="-10.16" size="1.778" layer="95" rot="R180">COM B</text>
<text x="7.62" y="15.24" size="1.778" layer="95" rot="R180">S1</text>
<text x="7.62" y="10.16" size="1.778" layer="95" rot="R180">A</text>
<text x="7.62" y="5.08" size="1.778" layer="95" rot="R180">B</text>
<text x="7.62" y="-2.54" size="1.778" layer="95" rot="R180">S2</text>
<text x="7.62" y="-7.62" size="1.778" layer="95" rot="R180">S3</text>
<text x="7.62" y="-12.7" size="1.778" layer="95" rot="R180">S4</text>
<text x="7.62" y="-17.78" size="1.778" layer="95" rot="R180">S5</text>
<pin name="COM_A" x="-12.7" y="7.62" visible="off" length="middle"/>
<pin name="COM_B" x="-12.7" y="-12.7" visible="off" length="middle"/>
<text x="-7.62" y="-22.86" size="1.778" layer="95">&gt;NAME</text>
<text x="-7.62" y="-25.4" size="1.778" layer="96">&gt;VALUE</text>
<circle x="-7.62" y="7.62" radius="0.254" width="0.254" layer="94"/>
<circle x="-7.62" y="-12.7" radius="0.254" width="0.254" layer="94"/>
</symbol>
</symbols>
<devicesets>
<deviceset name="SW_NAV_TSW" prefix="SW">
<description>C&amp;K TSWB-3N-CB111-LFS central select with jog and 4 directions (dial &amp; direction ring)&lt;br&gt;
&lt;br&gt;
white - TSWB-3N-CB111 LFS&lt;br&gt;
black - TSWB-3N-CB222 LFS&lt;br&gt;</description>
<gates>
<gate name="SW" symbol="TSWB3NCB111LFS" x="0" y="0"/>
</gates>
<devices>
<device name="" package="TSWB3NCB111LFS">
<connects>
<connect gate="SW" pin="A" pad="A"/>
<connect gate="SW" pin="B" pad="B"/>
<connect gate="SW" pin="COM_A" pad="COMA"/>
<connect gate="SW" pin="COM_B" pad="COMB"/>
<connect gate="SW" pin="S1" pad="S1"/>
<connect gate="SW" pin="S2" pad="S2"/>
<connect gate="SW" pin="S3" pad="S3"/>
<connect gate="SW" pin="S4" pad="S4"/>
<connect gate="SW" pin="S5" pad="S5"/>
</connects>
<technologies>
<technology name=""/>
</technologies>
</device>
</devices>
</deviceset>
</devicesets>
</library>
</libraries>
<attributes>
</attributes>
<variantdefs>
</variantdefs>
<classes>
<class number="0" name="default" width="0" drill="0">
</class>
</classes>
<parts>
<part name="SW1" library="we-switch" deviceset="SW_NAV_TSW" device=""/>
</parts>
<sheets>
<sheet>
<plain>
</plain>
<instances>
<instance part="SW1" gate="SW" x="73.66" y="86.36"/>
</instances>
<busses>
</busses>
<nets>
</nets>
</sheet>
</sheets>
</schematic>
</drawing>
</eagle>
