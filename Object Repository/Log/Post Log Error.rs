<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post Log Error</name>
   <tag></tag>
   <elementGuidId>dbaf3ef2-621d-42e2-a4d4-f52ba0df76c1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: 1,\n  \&quot;logDate\&quot;: \&quot;2019-05-28T02:39:20.084Z\&quot;,\n  \&quot;message\&quot;: \&quot;This is for test purpose only\&quot;,\n  \&quot;appCode\&quot;: \&quot;LOGMVC\&quot;,\n  \&quot;projectName\&quot;: \&quot;210.2.98.10\&quot;,\n  \&quot;className\&quot;: \&quot;string\&quot;,\n  \&quot;lineCode\&quot;: 0,\n  \&quot;logType\&quot;: 0,\n  \&quot;serverity\&quot;: 0,\n  \&quot;browser\&quot;: \&quot;FACEBOOK\&quot;,\n  \&quot;version\&quot;: \&quot;211.00\&quot;,\n  \&quot;location\&quot;: \&quot;Ho chi minh\&quot;,\n  \&quot;isActive\&quot;: true,\n  \&quot;exception\&quot;: {}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://apiunilog.unicode.edu.vn/api/Log/error</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
