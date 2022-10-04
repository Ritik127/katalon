//import com.katalon.KatalonHelper
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestSuiteContext
import com.kms.katalon.core.webui.keyword.builtin.CloseBrowserKeyword

class TestListener {
	/**
	 * Executes before every test suite starts.
	 * @param testSuiteContext: related information of the executed test suite.
	 */
		@AfterTestSuite
	def sampleBeforeTestSuite(TestSuiteContext testSuiteContext) {
		
		
	}
}