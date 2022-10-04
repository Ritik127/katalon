import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.click(findTestObject('Object Repository/cart_page/Page_Swag Labs/Page_Swag Labs/button_Add to cart'))

WebUI.rightClick(findTestObject('Object Repository/cart_page/Page_Swag Labs/Page_Swag Labs/button_Add to cart_1'))

WebUI.click(findTestObject('Object Repository/cart_page/Page_Swag Labs/Page_Swag Labs/button_Add to cart_1'))

WebUI.verifyElementText(findTestObject('Object Repository/cart_page/Page_Swag Labs/Page_Swag Labs/div_15.99'), '$15.99')

WebUI.verifyElementVisible(findTestObject('Object Repository/cart_page/Page_Swag Labs/Page_Swag Labs/button_Add to cart_1_2'))

WebUI.waitForElementVisible(findTestObject('Object Repository/cart_page/Page_Swag Labs/Page_Swag Labs/img_Add to cart_inventory_item_img'), 
    0)

