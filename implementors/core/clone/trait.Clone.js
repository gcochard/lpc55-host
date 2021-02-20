(function() {var implementors = {};
implementors["lpc55"] = [{"text":"impl Clone for Command","synthetic":false,"types":[]},{"text":"impl Clone for CommandTag","synthetic":false,"types":[]},{"text":"impl Clone for DataPhase","synthetic":false,"types":[]},{"text":"impl Clone for HidHeader","synthetic":false,"types":[]},{"text":"impl Clone for ResponseTag","synthetic":false,"types":[]},{"text":"impl Clone for Response","synthetic":false,"types":[]},{"text":"impl Clone for ReportId","synthetic":false,"types":[]},{"text":"impl Clone for Version","synthetic":false,"types":[]},{"text":"impl Clone for FlashReadMargin","synthetic":false,"types":[]},{"text":"impl Clone for Key","synthetic":false,"types":[]},{"text":"impl Clone for KeystoreOperation","synthetic":false,"types":[]},{"text":"impl Clone for ErrorGroup","synthetic":false,"types":[]},{"text":"impl Clone for GenericError","synthetic":false,"types":[]},{"text":"impl Clone for FlashDriverError","synthetic":false,"types":[]},{"text":"impl Clone for SbLoaderError","synthetic":false,"types":[]},{"text":"impl Clone for PropertyStoreError","synthetic":false,"types":[]},{"text":"impl Clone for CrcCheckerError","synthetic":false,"types":[]},{"text":"impl Clone for Properties","synthetic":false,"types":[]},{"text":"impl Clone for Property","synthetic":false,"types":[]},{"text":"impl Clone for AvailableCommands","synthetic":false,"types":[]},{"text":"impl Clone for AvailablePeripherals","synthetic":false,"types":[]},{"text":"impl Clone for PfrKeystoreUpdateOptions","synthetic":false,"types":[]},{"text":"impl Clone for IrqNotificationPin","synthetic":false,"types":[]},{"text":"impl Clone for Error","synthetic":false,"types":[]},{"text":"impl Clone for ProtectedFlash","synthetic":false,"types":[]},{"text":"impl&lt;CustomerData:&nbsp;Clone, VendorUsage:&nbsp;Clone&gt; Clone for FactorySettings&lt;CustomerData, VendorUsage&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;CustomerData: FactorySettingsCustomerData,<br>&nbsp;&nbsp;&nbsp;&nbsp;VendorUsage: FactorySettingsVendorUsage,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;CustomerData:&nbsp;Clone, VendorUsage:&nbsp;Clone&gt; Clone for CustomerSettings&lt;CustomerData, VendorUsage&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;CustomerData: CustomerSettingsCustomerData,<br>&nbsp;&nbsp;&nbsp;&nbsp;VendorUsage: CustomerSettingsVendorUsage,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;CustomerData:&nbsp;Clone, VendorUsage:&nbsp;Clone&gt; Clone for WrappedFactorySettings&lt;CustomerData, VendorUsage&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;CustomerData: FactorySettingsCustomerData,<br>&nbsp;&nbsp;&nbsp;&nbsp;VendorUsage: FactorySettingsVendorUsage,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;CustomerData:&nbsp;Clone, VendorUsage:&nbsp;Clone&gt; Clone for WrappedCustomerSettings&lt;CustomerData, VendorUsage&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;CustomerData: CustomerSettingsCustomerData,<br>&nbsp;&nbsp;&nbsp;&nbsp;VendorUsage: CustomerSettingsVendorUsage,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Clone for KeystoreHeader","synthetic":false,"types":[]},{"text":"impl Clone for Keycode","synthetic":false,"types":[]},{"text":"impl Clone for ActivationCode","synthetic":false,"types":[]},{"text":"impl Clone for Keystore","synthetic":false,"types":[]},{"text":"impl Clone for NxpArea","synthetic":false,"types":[]},{"text":"impl Clone for BootSpeed","synthetic":false,"types":[]},{"text":"impl Clone for IspMode","synthetic":false,"types":[]},{"text":"impl Clone for BootConfiguration","synthetic":false,"types":[]},{"text":"impl Clone for UsbId","synthetic":false,"types":[]},{"text":"impl Clone for TrustzoneMode","synthetic":false,"types":[]},{"text":"impl Clone for SecureBootConfiguration","synthetic":false,"types":[]},{"text":"impl Clone for PrinceConfiguration","synthetic":false,"types":[]},{"text":"impl Clone for PrinceSubregion","synthetic":false,"types":[]},{"text":"impl Clone for RawCustomerData","synthetic":false,"types":[]},{"text":"impl&lt;CustomerData:&nbsp;Clone, VendorUsage:&nbsp;Clone&gt; Clone for CustomerSettingsArea&lt;CustomerData, VendorUsage&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;CustomerData: CustomerSettingsCustomerData,<br>&nbsp;&nbsp;&nbsp;&nbsp;VendorUsage: CustomerSettingsVendorUsage,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Clone for Header","synthetic":false,"types":[]},{"text":"impl Clone for RawVendorUsage","synthetic":false,"types":[]},{"text":"impl Clone for FactorySettingsProgInProgress","synthetic":false,"types":[]},{"text":"impl Clone for RotKeysStatus","synthetic":false,"types":[]},{"text":"impl Clone for PrinceIvCode","synthetic":false,"types":[]},{"text":"impl Clone for MonotonicCounter","synthetic":false,"types":[]},{"text":"impl Clone for RotKeyStatus","synthetic":false,"types":[]},{"text":"impl Clone for DebugSecurity","synthetic":false,"types":[]},{"text":"impl Clone for DebugSecurityPolicy","synthetic":false,"types":[]},{"text":"impl Clone for DebugSecurityPolicies","synthetic":false,"types":[]},{"text":"impl Clone for SigningKeySource","synthetic":false,"types":[]},{"text":"impl Clone for Pki","synthetic":false,"types":[]},{"text":"impl Clone for WrappedPki","synthetic":false,"types":[]},{"text":"impl Clone for SigningKey","synthetic":false,"types":[]},{"text":"impl Clone for PublicKey","synthetic":false,"types":[]},{"text":"impl Clone for Signature","synthetic":false,"types":[]},{"text":"impl Clone for CertificateSlot","synthetic":false,"types":[]},{"text":"impl Clone for CertificateSource","synthetic":false,"types":[]},{"text":"impl Clone for Certificate","synthetic":false,"types":[]},{"text":"impl Clone for Certificates","synthetic":false,"types":[]},{"text":"impl Clone for Sha256Hash","synthetic":false,"types":[]},{"text":"impl Clone for BootTag","synthetic":false,"types":[]},{"text":"impl Clone for RawBootCommand","synthetic":false,"types":[]},{"text":"impl Clone for BootCommandDescriptor","synthetic":false,"types":[]},{"text":"impl Clone for BootCommand","synthetic":false,"types":[]},{"text":"impl Clone for Config","synthetic":false,"types":[]},{"text":"impl Clone for Firmware","synthetic":false,"types":[]},{"text":"impl Clone for Reproducibility","synthetic":false,"types":[]},{"text":"impl Clone for Filetype","synthetic":false,"types":[]},{"text":"impl Clone for Sb21FileParameters","synthetic":false,"types":[]},{"text":"impl Clone for UnsignedSb21File","synthetic":false,"types":[]},{"text":"impl Clone for Sb21CommandPart","synthetic":false,"types":[]},{"text":"impl Clone for SignedSb21File","synthetic":false,"types":[]},{"text":"impl Clone for Sb21HeaderPart","synthetic":false,"types":[]},{"text":"impl Clone for Version","synthetic":false,"types":[]},{"text":"impl Clone for Keyblob","synthetic":false,"types":[]},{"text":"impl Clone for Sb2Header","synthetic":false,"types":[]},{"text":"impl Clone for FullCertificateBlockHeader","synthetic":false,"types":[]},{"text":"impl Clone for HttpConfig","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()