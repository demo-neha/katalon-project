<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>testing</description>
   <name>GetCountryInfo</name>
   <tag></tag>
   <elementGuidId>0eed7d68-c05d-4044-aaaa-62ba6ed6b05a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;CountryCurrency xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;>
            &lt;sCountryISOCode>IN&lt;/sCountryISOCode>
        &lt;/CountryCurrency>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>ListOfContinentsByName</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementsCount(response, 'issues', 6)
WS.verifyEqual(jsonResponse.issues.size, 6)

assertThat(jsonResponse.issues.size).isEqualTo(6)
assertThat(jsonResponse.issues).hasSize(6)

WS.verifyGreaterThan(jsonResponse.issues.size, 5)
assertThat(jsonResponse.issues.size).isGreaterThan(5)

assertThat(jsonResponse.issues.size).isBetween(5,7)

assertThat(response.getResponseText()).contains('Katalon Test Project')

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

assertThat(response.getResponseText()).contains('Katalon Test Project')

assertThat(response.getResponseText()).contains('Katalon Test Project')

assertThat(response.getResponseText()).contains('Katalon Test Project')
</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
