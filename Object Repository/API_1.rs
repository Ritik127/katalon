<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_1</name>
   <tag></tag>
   <elementGuidId>279b99b7-ad5a-40a8-a5a3-a692885afc94</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;morpheus\&quot;,\n    \&quot;job\&quot;: \&quot;leader\&quot;\n}&quot;,
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
      <webElementGuid>548430bc-da09-4c97-a9b9-59183f061125</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=${number}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'api/users'</defaultValue>
      <description></description>
      <id>2f65148c-2b8c-419b-a9f4-e41023e6f1d9</id>
      <masked>false</masked>
      <name>uri</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>249960a0-e665-4206-a00f-0d9c60e7a634</id>
      <masked>false</masked>
      <name>number</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

//RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



RequestObject request = WSResponseManager.getInstance().getCurrentRequest()


//assertThat(response.getResponseText()).contains('michael.lawson@reqres.in')


//assertThat(response.getResponseText()).isEqualTo(&quot;michael.lawson@reqres.in&quot;)


//WS.verifyElementPropertyValue(response, 'data[1].email', 'lindsay.ferguson@reqres.in')


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)







GlobalVariable.variable =WS.getElementPropertyValue(response, 'id')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
