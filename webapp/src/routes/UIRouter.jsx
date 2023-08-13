import { Route, Routes, BrowserRouter } from "react-router-dom";
import {
  Produce,
  AccountSignUp,
  AccountLogin,
  AccountConfirm,
  PasswordForgot,
  PasswordReset,
  PasswordChange,
  PersonalInfo,
  PersonalInfoUpdate,
  UserProfile,
  UserProfileUpdate,
  UserProfilePhoto,
  EmailChange,
  EmailChangeApprove,
  EmailChangeConfirm,
  Harvest,
  HarvestCreate,
  HarvestUpdate,
  FarmCreate,
  Farm,
  Location,
  FarmUpdate,
  FarmLogoUpload,
  LocationCreate,
  LocationUpdate,
  DirectMessage,
  PageNotFound,
  Conversations,
} from "../views";

export function UIRouter() {
  return (
    <BrowserRouter>
      <Routes>
        {/* Produce routes */}
        <Route path="/" element={<Produce />} />
        <Route path="/produce" element={<Produce />} />
        <Route path="/list-a-produce" element={<HarvestCreate />} />
        <Route path="/produce/:harvest_id" element={<Harvest />} />
        <Route path="/produce/:harvest_id/edit" element={<HarvestUpdate />} />

        {/* Farmer routes */}
        <Route path="/become-a-farmer" element={<FarmCreate />} />
        <Route path="/farm/:farm_id" element={<Farm />} />
        <Route path="/farm/:farm_id/edit" element={<FarmUpdate />} />
        <Route path="/farm/:farm_id/logo" element={<FarmLogoUpload />} />
        <Route
          path="/farm/:farm_id/add-location"
          element={<LocationCreate />}
        />
        <Route path="/farm/location/:location_id" element={<Location />} />
        <Route
          path="/farm/location/:location_id/edit"
          element={<LocationUpdate />}
        />

        {/* Accounts routes */}
        <Route path="/signup" element={<AccountSignUp />} />
        <Route path="/login" element={<AccountLogin />} />
        <Route path="/account-confirm" element={<AccountConfirm />} />
        <Route path="/forgot-password" element={<PasswordForgot />} />
        <Route path="/reset-password" element={<PasswordReset />} />

        <Route
          path="/account/setting/change-password"
          element={<PasswordChange />}
        />
        <Route path="/account/setting/change-email" element={<EmailChange />} />
        <Route
          path="/account/setting/approve-email-change"
          element={<EmailChangeApprove />}
        />
        <Route
          path="/account/setting/confirm-email"
          element={<EmailChangeConfirm />}
        />
        <Route
          path="/account/setting/personal-info"
          element={<PersonalInfo />}
        />
        <Route
          path="/account/setting/personal-info-edit"
          element={<PersonalInfoUpdate />}
        />
        <Route path="/user/profile" element={<UserProfile />} />
        <Route
          path="/user/profile/upload-photo"
          element={<UserProfilePhoto />}
        />
        <Route path="/user/:user_id/profile" element={<UserProfile />} />
        <Route
          path="/user/:user_id/profile/edit"
          element={<UserProfileUpdate />}
        />

        {/* DirectMessages/Chat pages */}
        <Route path="/user/chat/" element={<DirectMessage />} />
        <Route path="/user/chat/conv" element={<Conversations />} />

        {/* Page Not Found */}
        <Route path="/*" element={<PageNotFound />} />
      </Routes>
    </BrowserRouter>
  );
}
