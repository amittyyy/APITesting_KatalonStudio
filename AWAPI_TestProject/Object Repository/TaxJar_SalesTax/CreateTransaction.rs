<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateTransaction</name>
   <tag></tag>
   <elementGuidId>08d15f4f-922d-480d-acb4-00fc686e93c0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;transaction_id\&quot;: \&quot;01020221\&quot;,\n  \&quot;transaction_date\&quot;: \&quot;2022-01-03T00:51:29.257Z\&quot;,\n  \&quot;to_state\&quot;: \&quot;NY\&quot;,\n  \&quot;to_city\&quot;: \&quot;Dix Hills\&quot;,\n  \&quot;to_zip\&quot;: \&quot;11746\&quot;,\n  \&quot;amount\&quot;: 5,\n  \&quot;shipping\&quot;: 0,\n  \&quot;sales_tax\&quot;: 0.04,\n  \&quot;product_tax_code\&quot;: \&quot;81162000A9000\&quot;,\n  \&quot;description\&quot;: \&quot;amity test only\&quot;,\n  \&quot;product_identifier\&quot;: \&quot;PP\&quot;\n}&quot;,
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
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dev-awpurchaseapi.accountantsoffice.com/api/salesTaxRate/createOrderSelfService</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
