<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Change Password</name>
   <tag></tag>
   <elementGuidId>667350a9-1663-4093-b0e9-53e93acc345f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;vmdung450@gmail.com\&quot;,\n  \&quot;currentPassword\&quot;: \&quot;Zaq@1234\&quot;,\n  \&quot;newPassword\&quot;: \&quot;zaQ@1234\&quot;,\n  \&quot;confirmPassword\&quot;: \&quot;zaQ@1234\&quot;,\n  \&quot;token\&quot;: \&quot;CfDJ8PYR7J8xVONKv4EesiPIu0JKcV1k/l3Li+975yO6YJdqxrW6ainQKJCqE8rmsYOxYoaHeLxndsm9WBgGFC3tKGQT69A7hRR+wZx6mO8AzGZyD4dI4pNVrbDRJaafae31aOJeTtDB0/oqXUMZSUrJqOap3G57DGPlr5dLe/q5fblq4xmLmjjgI9HcJX8gURFukQ\u003d\u003d\&quot;\n}&quot;,
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
   <restUrl>http://apiunilog.unicode.edu.vn/api/Password</restUrl>
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