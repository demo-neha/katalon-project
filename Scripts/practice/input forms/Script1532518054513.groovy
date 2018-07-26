import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('http://www.seleniumeasy.com/test/basic-first-form-demo.html')

WebUI.getText(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/a_Input Forms'))

WebUI.click(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/a_Input Forms'))

WebUI.waitForElementPresent(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/a_Simple Form Demo'), 
    40)

not_run: WebUI.click(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/a_Simple Form Demo'))

WebUI.setText(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/input_user-message'), 'neha mehakl')

not_run: WebUI.setText(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/input_sum1'), '10')

not_run: WebUI.setText(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/input_sum2'), '20')

not_run: WebUI.getText(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/button_Get Total'))

not_run: WebUI.closeBrowser()

WebUI.waitForElementPresent(findTestObject('New Folder/Page_Selenium Easy Demo - Simple Fo/a_Checkbox Demo'), 40)

WebUI.click(findTestObject('New Folder/Page_Selenium Easy Demo - Simple Fo/a_Checkbox Demo'))

not_run: WebUI.waitForElementPresent(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/a_Simple Form Demo'), 
    40)

not_run: WebUI.click(findTestObject('Object Repository/New Folder/Page_Selenium Easy - Checkbox demo/a_Table'))

not_run: WebUI.waitForElementPresent(findTestObject('Object Repository/New Folder/Page_Selenium Easy Demo - Simple Fo/a_Simple Form Demo'), 
    40)

not_run: WebUI.click(findTestObject('Object Repository/New Folder/Page_Selenium Easy - Checkbox demo/a_Table Pagination'))

not_run: WebUI.click(findTestObject('Object Repository/New Folder/Page_Selenium Easy - Table with Pag/a_Alerts  Modals'))

not_run: WebUI.click(findTestObject('Object Repository/New Folder/Page_Selenium Easy - Table with Pag/a_Bootstrap Alerts'))

