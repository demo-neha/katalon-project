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

WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/')

WebUI.setText(findTestObject('Page_OrangeHRM/input_txtUsername'), 'Admin')

WebUI.setEncryptedText(findTestObject('Page_OrangeHRM/input_txtPassword'), 'hUKwJTbofgPU9eVlw/CnDQ==')

WebUI.click(findTestObject('Page_OrangeHRM/input_Submit'))

WebUI.click(findTestObject('Page_OrangeHRM/b_PIM'))

WebUI.click(findTestObject('Page_OrangeHRM/a_Employee List'))

WebUI.setText(findTestObject('Page_OrangeHRM/input_empsearchemployee_nameem'), 'neha mehlacc')

WebUI.setText(findTestObject('Page_OrangeHRM/input_empsearchid'), '5962')

WebUI.selectOptionByValue(findTestObject('Page_OrangeHRM/select_AllFreelanceFull-Time C'), '2', true)

WebUI.selectOptionByValue(findTestObject('Page_OrangeHRM/select_AllSalesAdministrationI'), '4', true)

WebUI.selectOptionByValue(findTestObject('Page_OrangeHRM/select_Current Employees OnlyC'), '2', true)

WebUI.selectOptionByValue(findTestObject('Page_OrangeHRM/select_AllAccount ClerkCEOFina'), '9', true)

WebUI.setText(findTestObject('Page_OrangeHRM/input_empsearchsupervisor_name'), 's')

WebUI.doubleClick(findTestObject('Page_OrangeHRM/fieldset_Employee Name        '))

WebUI.click(findTestObject('Page_OrangeHRM/input__reset'))

